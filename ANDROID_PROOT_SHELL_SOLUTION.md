# Android App 内置 PRoot 沙箱架构下 Shell、MCP、临时目录与网络基础设施解决方案指南

## 📌 一、 背景与架构概述

在将 `codex-rs` Agent 核心移植部署至 Android 移动端时，Android App 内部嵌入了标准的开源 **PRoot 容器引擎**（基于官方 `proot-me/PRoot` 项目为 Android ARM64 交叉编译）。

PRoot 容器为嵌入在 App 内部的 Rust Agent 核心提供了一个完整的、无须 Root 权限的 Linux POSIX 运行环境。本指南针对 Rust Agent 中 `/bin/sh` 命令派生、本地 Stdio MCP (Node.js/Python) 服务的派生连通、临时文件驻留 Android App 内部存储，以及代理/TLS 根证书/DNS 域名解析等基础通信基础设施提供权威的技术诊断与落地规范。

---

## 🔍 二、 核心问题诊断与原理剖析

### 1. `/bin/sh` 路径运行机制与不可修改原因
- **POSIX FHS 标准与 Android OS 差异**：裸 Android OS 无 `/bin/sh`（位于 `/system/bin/sh`）。
- **PRoot 虚拟化拦截**：开源 PRoot 通过 `ptrace` 系统调用拦截器（`syscall.c` / `path.c`）自动拦截 `execve("/bin/sh", ...)` 调用，并将其透明重定向至 Guest RootFS 内部的路径（如 `<app_files>/rootfs/bin/sh`）。
- **避坑说明**：若盲目将 Rust 源码中的路径改为 `/system/bin/sh`，运行在 PRoot 容器内部的进程反而会因为 Guest RootFS 内部不存在 `/system/bin/sh` 而抛出 `execve` 找不到文件的错误。
- **结论**：**Rust 源码中的 `/bin/sh` 绝对路径在 PRoot 沙箱内部是 100% 标准且正确的，切勿修改 Rust 源码！**

---

## ⚙️ 三、 官方 PRoot 运行 `/bin/sh` 的 3 大坑点与解决方案

在 Android App 启动嵌入的 PRoot 容器时，必须在 App 宿主层保证以下 3 个关键配置：

### 坑点 1：`LD_PRELOAD` 环境变量污染导致 `execve` 失败
- **现象**：报错 `proot error: execve(...): No such file or directory`。
- **原因**：若 Android 宿主环境或父进程中设置了 `LD_PRELOAD`（例如 Android 系统的 Hook 动态库），会优先拦截 `execve` 系统调用，干扰 PRoot 的 `ptrace` 机制。
- **解决方案**：App 在启动 PRoot 命令行之前，必须显式清空 `LD_PRELOAD` 环境变量：
  ```bash
  unset LD_PRELOAD
  # 或者在 ProcessBuilder 中 env.remove("LD_PRELOAD")
  ```

### 坑点 2：Android 10+ 内核 `seccomp` 限制导致 `ptrace` 被拦截
- **现象**：报错 `proot error: Function not implemented` 或 `Permission denied`。
- **原因**：Android 10+ 针对无 Root 应用应用了更严格的 `seccomp` 过滤规则，限制了部分 `ptrace` 追踪操作。
- **解决方案**：在 App 启动 PRoot 容器时注入 `PROOT_NO_SECCOMP=1` 环境变量，强制 PRoot 使用纯 `ptrace` 模式避开 `seccomp` 拦截：
  ```bash
  export PROOT_NO_SECCOMP=1
  ```

### 坑点 3：Guest RootFS 的 ELF 动态链接器缺失
- **现象**：路径存在但执行 `execve("/bin/sh")` 依然报错 `No such file or directory`。
- **原因**：`/bin/sh` 通常是动态链接文件（指向 BusyBox 或 bash/ash），内核在处理 `execve` 时需要加载 Guest RootFS 内部的动态链接器（如 `/lib/ld-linux-aarch64.so.1` 或 `/lib/ld-musl-aarch64.so.1`）。
- **解决方案**：确保解压在 App 私有区目录的 RootFS（如 Alpine / Debian 根镜像）完整无损，包含 `/lib` 与 `/lib64` 路径下的动态链接库。

---

## 🔌 四、 本地 Stdio MCP (Node.js/Python) 通过 PRoot 派生与连通机制

### 1. 运作原理与可行性分析
- **`rmcp-client` 底层**：Codex `rmcp-client` 在部署本地 MCP 服务时，会读取配置的命令行字符串（如 `npx -y @modelcontextprotocol/server-filesystem` 或 `python mcp_server.py`），使用 `tokio::process::Command` 派生子进程，并建立异步匿名管道 (`ChildStdin` / `ChildStdout`) 进行 JSON-RPC 报文交互。
- **PRoot 管道透明转发**：
  1. **子进程派生**：当 Rust Agent 核心在 PRoot 环境下启动 Node.js 或 Python MCP 服务时，PRoot 通过 `ptrace` 拦截 `execve`，在容器 RootFS (`/usr/bin/node` 或 `/usr/bin/python`) 中匹配并运行二进制。
  2. **管道通透性**：父子进程之间的 `stdin` / `stdout` 匿名管道走内核层 pipe buffer，由 PRoot 透明保留并转发。
- **结论**：**完全可行，100% 兼容！** 在 PRoot 内已安装 Node.js / Python 工具链的情况下，无需任何回退或改动代码，即可直接派生并连通本地 Stdio MCP 服务。

---

## 📂 五、 临时文件驻留 Android App 内部存储 (`Context.getCacheDir()`) 方案

### 1. 为什么临时目录必须存放在 App 内部（而不是 PRoot 镜像中）？
- **Android 原生生命周期管理**：将临时目录存放在 `/data/data/<package_name>/cache`（即 `context.cacheDir`），可享受 Android 原生系统缓存清理机制。用户清理缓存或卸载 App 时，系统会自动回收。
- **Kotlin 与 Rust 无缝共享**：Android App 前台 UI (Kotlin/Java) 可直接通过 `File(context.cacheDir, "...")` 读写或分析 Agent 产生的临时文件与日志，无需跨越 PRoot 挂载边界。
- **解耦 PRoot 镜像重置**：重置或替换 PRoot RootFS 时，Agent 运行日志与临时状态不会随之丢失。

### 2. Rust 标准库 Android 特性与 JNI 注入方案
- **Rust 标准库坑点**：根据 Rust 官方实现（`std::env::temp_dir()`），在 Android 平台上若未指定 `$TMPDIR` 环境变量，Rust 默认会退回使用 `/data/local/tmp`。但非 Root 应用**没有权限写入 `/data/local/tmp`**，会抛出 `PermissionDenied`。
- **落地方案（JNI 环境变量注入）**：
  在 App 初始化并启动 Rust Agent 核心（JNI `.so` 加载）的第一时间，由 Kotlin/Java 显式读取 `context.cacheDir.absolutePath` 并注入环境变量：
  ```kotlin
  // Kotlin JNI 初始化层
  val cachePath = context.cacheDir.absolutePath // /data/data/<pkg>/cache
  android.system.Os.setenv("TMPDIR", cachePath, true)
  ```
  注入后，Rust 标准库 `std::env::temp_dir()` 与 `tempfile::tempdir()` 会 100% 自动锁定并使用该路径，实现临时文件天然驻留在 App 内部存储中。

---

## 💻 六、 PRoot-Linux 沙箱环境下的 `shell-command` 命令行支持

### 1. 架构兼容性说明
- 在 Android App 内部集成 PRoot-Linux 沙箱并结合 App 内置终端视图（In-App Terminal View）时，用户与 Agent 需要在 PRoot 容器内部派生 Shell 命令行并交互。
- **`shell-command` 模块的核心作用**：
  - 负责派生子进程、管理 PGID 进程组、重定向 stdin/stdout 管道以及响应终端尺寸重置信号（`SIGWINCH`）。
- **结论**：结合 App 的 PRoot-Linux 沙箱，`shell-command` 具备完全真实的运行场景，**100% 完整保留并正常提供服务**！

---

## 🌐 七、 Android 网络基础设施（代理、TLS CA、DNS）落地方案

### 1. 网络代理感知（`http-client/src/outbound_proxy.rs`）
- **解决方案**：非 Windows/macOS 平台（如 Android）下，桌面 `sys_proxy` 检测自动禁用，退回为标准的 `reqwest` 环境变量代理解析（`HTTPS_PROXY` / `HTTP_PROXY` / `ALL_PROXY` / `NO_PROXY`）。
- **使用规范**：当 Android 手机开启科学上网、VPN 或 Charles/Clash 抓包代理时，App 层通过 JNI 设置环境变量 `HTTP_PROXY=http://127.0.0.1:7890`，Agent 会自动将所有大模型 API 流量通过代理转发。

### 2. TLS CA 根证书保底回退（`utils/rustls-provider` & `http-client`）
- **解决方案**：已在 `http-client/src/custom_ca.rs` 中增加了 `webpki-roots` 编译期根证书库保底机制：
  ```rust
  let _ = root_store.add_parsable_certificates(certs);
  if root_store.is_empty() {
      root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
  }
  ```
- **生效保障**：当运行在缺乏 `/etc/ssl/certs/ca-certificates.crt` 的 Android 系统上时，系统根证书加载失败会自动回退使用 Mozilla 官方编译期打入的根证书库，消除了 `CertWebPKIError(UnknownIssuer)` 握手失败报错。

### 3. DNS 域名解析（`http-client` / `backend-client`）
- **解决方案**：`codex-rs` 全库 DNS 解析统一使用 Tokio 标准的 `tokio::net::lookup_host`，在 Android (`target_os = "android"`) 上原生由 Bionic C 库的 `libc::getaddrinfo` 驱动，直连 Android 底层 `netd` 守护进程，避开任何读取 `/etc/resolv.conf` 的纯 Rust 库。

---

## 📋 八、 总结与最佳实践 checklist

1. **Rust 源码层面**：
   - 保持 `codex-rs` 中 `hooks/src/engine/command_runner.rs`、`shell-command/src/shell_detect.rs` 等模块现有的 `/bin/sh` 标准路径不变。
   - 保持 `shell-command` 在 PRoot-Linux 沙箱下的命令行派生与 PTY 交互能力。
   - 保持 `rmcp-client` 的 Stdio 本地 MCP 派生能力不变。
   - 物理删除 `tui` 控制台 UI 模块。
   - `realtime_prompt.rs` 补充 Android `u0_a*` 隔离 UID 软过滤。

2. **Android App 接入层 Checklist**：
   - [x] 指定根目录挂载：`proot -r /data/data/<pkg>/files/rootfs`
   - [x] 挂载必要的伪文件系统：`-b /dev -b /proc -b /sys`
   - [x] 清除环境变量污染：`unset LD_PRELOAD`
   - [x] 启用无 seccomp 兼容模式：`export PROOT_NO_SECCOMP=1`
   - [x] JNI 注入 App 缓存路径：`Os.setenv("TMPDIR", context.cacheDir.absolutePath, true)`
   - [x] JNI 注入移动端代理（如需）：`Os.setenv("HTTP_PROXY", proxyUrl, true)`
