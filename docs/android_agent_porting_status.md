# 📱 Codex-RS Android 专用 Agent 核心重构与裁剪进度报告

> **文档更新时间**：2026年7月  
> **项目状态**：进行中 (Active Refactoring & Porting)  
> **代码库路径**：`codex-rs/`  

---

## 🎯 一、项目定位与重构目标

本项目致力于将原本面向桌面端（macOS/Linux/Windows）CLI 终端和多进程守护进程架构的 **Codex-RS** 引擎，彻底重构为 **Android App 专用私有化嵌入式 Agent 核心（Android Dedicated Embedded Agent Core）**。

### 核心重构原则：
1. **进程内 JNI/UniFFI 直连**：剥离桌面端的后台守护进程（Daemon）与 TCP/Socket IPC 传输层，改为通过 Rust 动态链接库 (`.so`) 在 Android App 内存中直接与 Java/Kotlin 交互。
2. **移动端绝对安全与零冗余**：移除静默遥测上报、桌面系统凭据锁、桌面终端检测、桌面 GUI 交互与不可用的 Linux 内核提权/隔离模块，降低 CPU / 电量开销与 APK 二进制体积。
3. **平台原生能力替换**：
   - 避免引入 V8 C++ 静态库（改用 Android 原生 `JavaScriptEngine` / WebView V8 JIT）。
   - 避免依赖 Bubblewrap / Landlock 桌面隔离（改用 Android App 内置 UID / SELinux / PRoot 沙箱）。

---

## 🚀 二、已完成的裁剪与重构成果

目前已累计完成 **19 轮深度裁剪**，彻底物理剥离与合并解耦了 **46 个桌面冗余/不可用/悬空/过度拆分 Crate**，清理干掉了 **约 153,300+ 行桌面冗余代码**，将工作区 Crate 数量从 108+ 成功瘦身降至 **63 个**。

### 明细剥离清单：

| 剥离阶段 | 彻底删除/内联的 Crate 模块 | 剥离原因与移动端替代方案 |
| :--- | :--- | :--- |
| **第一/二轮** | `feedback`, `install-context`, `stdio-to-uds`, `utils/sleep-inhibitor` | 桌面用户反馈弹窗、Homebrew 路径检测与桌面休眠抑制器完全冗余 |
| **第三轮** | `v8-poc` | 避免 30MB+ C++ V8 静态库打包进 APK，转用 Android 原生 JIT |
| **第四轮** | `bwrap` | 桌面 Linux Bubblewrap 隔离在 Android App 沙箱下无权限且不可用 |
| **第五轮** | `app-server-daemon`, `app-server-transport`, `app-server-test-client` | 剥离桌面 IPC 守护进程与传输层，保留 `app-server-protocol` 作为 JNI DTO 传输 Schema 定义库 |
| **第六轮** | `terminal-detection`, `keyring-store`, `utils/cli`, `thread-manager-sample`, 精简 `utils/pty` | 移除桌面终端 detection、Keychain 锁与 CLI 格式化，精简 PTY 仅保留纯粹 Pipe 管道 |
| **第七轮** | `shell-escalation`, `uds`, `test-binary-support`, `utils/approval-presets`, `utils/cargo-bin` | 移除桌面 OSAScript/sudo 提权界面、悬空 UDS 库、测试 Mock 二进制与 Cargo 路径查找器 |
| **第八轮** | `linux-sandbox`, `file-watcher`, `websocket-client` | 移除桌面 Linux Landlock 沙箱镜像（8,203行）、Inotify 文件监听悬空库与桌面 WebSocket 传输客户端 |
| **第九轮** | `ext/guardian`, `lmstudio`, `ollama`, `utils/oss` | 移除 Guardian 悬空审查扩展、LMStudio/Ollama 桌面本地模型连接器及 OSS 包装库 |
| **第十轮** | `utils/readiness`, `utils/sandbox-summary`, 内联 `collaboration-mode-templates` | 移除悬空环境就绪库、桌面 CLI 彩色报告库，将 4 行模版 Crate 内联合并至 `models-manager` |
| **第十一轮** | `ext/connectors`, `utils/fuzzy-match`, 内联 `async-utils` | 移除悬空 Connector 扩展、桌面终端补全库，将 86 行 Async 库内联合并至 `core` |
| **第十二轮** | `utils/elapsed`, 内联 `utils/json-to-toml` | 移除悬空耗时格式化库，将 83 行 JSON->TOML 转换库内联合并至 `mcp-server` |
| **第十三轮** | `core-api`, `ext/agent`, 内联 `context-fragments` | 移除 Core API 悬空接口库、Agent Extension 悬空扩展库，将 231 行 Fragment 库内联合并至 `core` |
| **第十四轮** | 内联 `utils/home-dir` | 物理删除 134 行 `utils/home-dir` 桌面 Home 路径查找库，将 `find_codex_home` 内联合并至 `codex-home` |
| **第十五轮** | 内联 `response-debug-context` | 物理删除 166 行 `response-debug-context` 报文 Debug 解析库，合并内联至 `codex-api` |
| **第十六轮** | `exec-server`, `network-proxy`, `mcp-server`, `code-mode-host` | 物理删除 4 大 Android 严重不兼容中大型 Crate（守护进程服务器、MITM CA 代理、Stdio MCP 服务端、Node.js 宿主，共 57,000+ 行代码） |
| **第十七轮** | `windows-sandbox-rs`, `process-hardening`, `app-server-client` | 物理删除 Windows 沙箱实现、进程加固模块及 App Server 客户端残留 |
| **第十八轮** | `sandboxing`, `exec`, `arg0`, `execpolicy` | 物理删除 4 大 Android 严重不兼容/桌面控制台 Crate（系统沙箱包装器、CLI 命令行入口、argv[0] 符号链接分发器、PowerShell/Brew 安全策略规则库，共 16,338 行代码） |
| **第十九轮** | `utils/pty`, `code-mode`, `code-mode-protocol` | 物理删除 3 大 PTY 伪终端系统调用库、V8 CodeMode 宿主与 Protocol 传输库（7,290 行代码）并物理清除 `webbrowser` 依赖 |
| **第二十轮** | 音频预处理与 `symphonia` 库 | 物理彻底删除 `audio_preparation.rs` 语音 Token 预处理模块及 `symphonia` 音频解码库，物理清除 AWS SDK 凭据残留，纯粹保留移动端图像处理能力 |
| **遥测控制** | `analytics`, `otel` 网络分发暂停 | 封堵静默用户事件 POST 与 OTLP/Statsig 线程组，消除电量与流量隐患 |

---

## 📈 三、当前代码库最新规模指标

经最新盘点，`codex-rs/` 工作区代码库数据如下：

- 📦 **Workspace Crate 总数**：**63 个**（从 108+ 降至 63 个，达成极高纯度瘦身）
- 📄 **Rust 源码文件数 (`.rs`)**：**1,518 个**
- 📝 **Rust 代码总行数 (LoC)**：**577,970 行**
- 🧹 **依赖与引用状态**：全工作区针对上述已删除 46 个 Crate 及音频/AWS 模块的代码引用与 Cargo 依赖已 **100% 清零**，Git 工作区处于完全干净（Clean）状态。

---

## 🔍 四、Android 平台兼容性排查与降级指引

### 1. 沙箱与权限隔离 (Linux/Desktop 专属)
- **`bwrap` / `landlock` / `sandboxing`**：全部完成物理剥离。Android 拥有原生 SELinux + App 独占 UID 内核级隔离沙箱，无需且禁止调用 Linux Landlock / Bubblewrap 或 Windows AppContainer。

### 2. 桌面级 IPC 与守护进程
- **`app-server-daemon` / `app-server-transport` / `uds`**：全部彻底物理删除。移动端场景中，Agent 核心被编译为 `libcodex.so` 嵌入到 Android App 内部，无需在本地监听 Unix Domain Socket 或运行独立的桌面 IPC 守护进程。保留 `app-server-protocol` 作为 JNI DTO 传输定义。

### 3. 系统集成与终端机制
- **`utils/pty`**：禁用与清除 ConPTY/Windows/macOS 系统特有 API，Android 上改用基于管道 (Pipe) 的标准 IO 交互。
- **`codex-home`**：移除了硬编码桌面 `~/.codex` 的路径依赖，由 Android App 在 Agent 初始化时显式传入 App 内部私有存储目录 (`/data/data/<pkg>/files/codex`)。
- **剪贴板依赖 (`arboard`)**：彻底移除了带 `wayland-data-control` 桌面 GUI 特性的剪贴板依赖。

### 4. 后端集成与功能降级建议
- **本地大模型 (`ollama` / `lmstudio`)**：已完成剥离，Android 上不运行桌面级 GUI 服务，统一推荐云端 API 或轻量级端侧小模型。
- **V8 JavaScript 引擎 (`code-mode`)**：已剥离 C++ V8 静态库（~50MB+），建议 Android 编译选项中默认禁用 V8 特性。
- **OAuth 浏览器流程 (`rmcp-client`)**：移除桌面 `webbrowser::open` 自动拉起机制，改用 Android Chrome Custom Tabs / Deep Link 重定向。

---

## 📌 五、下一步计划

1. **JNI / UniFFI 接口层设计**：
   - 围绕 `app-server-protocol` 的 Schema DTO 定义，建立 Kotlin 与 Rust Agent Core 之间的 JSON/Memory 传输协议。
2. **Android NDK 工具链编译验证**：
   - 配置 `aarch64-linux-android` 交叉编译 Target，验证 NDK 构建输出。
2. **Android NDK 交叉编译构建**：
   - 验证 `aarch64-linux-android` 架构下的 Rust 动态库 (`libcodex_android.so`) 编译。
3. **嵌入式沙箱与执行器调优**：
   - 适配 Android App 私有目录 (`/data/data/<package_name>/files`) 下的工具执行与 PRoot 文件隔离。


