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

目前已累计完成 **12 轮深度裁剪**，彻底物理剥离与合并解耦了 **28 个桌面冗余/不可用/悬空/过度拆分 Crate**，清理干掉了 **66,000+ 行桌面冗余代码**，将工作区 Crate 数量从 108+ 成功瘦身降至 **80 个**。

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
| **遥测控制** | `analytics`, `otel` 网络分发暂停 | 封堵静默用户事件 POST 与 OTLP/Statsig 线程组，消除电量与流量隐患 |

---

## 📈 三、当前代码库最新规模指标

经最新盘点，`codex-rs/` 工作区代码库数据如下：

- 📦 **Workspace Crate 总数**：**80 个**（从 108+ 降至 80 个，达成极高纯度瘦身）
- 📄 **Rust 源码文件数 (`.rs`)**：**1,756 个**
- 📝 **Rust 代码总行数 (LoC)**：**715,486 行**
- 🧹 **依赖与引用状态**：全工作区针对上述已删除 28 个 Crate 的代码引用与 Cargo 依赖已 **100% 清零**，Git 工作区处于完全干净（Clean）状态。

---

## 📌 四、当前正在推进与下一步计划

1. **JNI / UniFFI 接口层设计**：
   - 围绕 `app-server-protocol` 的 Schema DTO 定义，建立 Kotlin 与 Rust Agent Core 之间的 JSON/Memory 传输协议。
2. **Android NDK 交叉编译构建**：
   - 验证 `aarch64-linux-android` 架构下的 Rust 动态库 (`libcodex_android.so`) 编译。
3. **嵌入式沙箱与执行器调优**：
   - 适配 Android App 私有目录 (`/data/data/<package_name>/files`) 下的工具执行与 PRoot 文件隔离。
