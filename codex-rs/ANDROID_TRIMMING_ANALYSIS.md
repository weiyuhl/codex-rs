# Codex-RS Android 裁剪分析文档

> **项目目标**：将 Codex-RS 引擎裁剪为 Android App 专用嵌入式 Agent 核心  
> **关键前提**：Android App 内嵌 **Proot-Linux 沙箱**，提供完整 Linux 用户态环境  
> **当前阶段**：物理删除不可用/不适用模块（非重构适配）  
> **文档状态**：分析整理阶段，暂不执行裁剪操作

---

## 一、已完成的裁剪（代码中已有 REMOVED-DELETED-CRATE 标记）

| 已删除 Crate | 原职责 | 残留引用位置 |
|---|---|---|
| `codex_exec_server` | 多进程执行服务器、环境管理、Noise 协议通信 | `core/src/session/tests.rs`、`guardian_tests.rs`、`ext/mcp/src/lib.rs`、`ext/skills/src/provider/executor.rs` 等 |
| `codex_network_proxy` | 网络代理、域名策略裁决、代理环境变量 | `core/src/session/mod.rs`、`core/src/session/tests.rs`、`core/tests/suite/realtime_conversation.rs` 等 |

> 这些 crate 已从 workspace 中物理删除，仅余 `// REMOVED-DELETED-CRATE:` 注释标记。

---

## 二、确认保留的模块（Proot 内可正常工作）

| 模块/Crate | 理由 |
|---|---|
| `shell-command` | Proot 内正常执行 shell 命令 |
| `portable-pty`（依赖） | Proot 内 PTY 终端可用 |
| `git-utils` | Proot 内 git 正常运行 |
| `apply-patch` | 文件补丁在 Proot 文件系统内正常 |
| `file-search`（nucleo） | 文件遍历/模糊搜索在 Proot 内正常 |
| `hooks` | 外部命令 Hook 在 Proot 内执行 |
| `codex-mcp`（MCP 子进程管理） | 子进程在 Proot 内启动运行 |
| `file-system` | 文件操作在 Proot 内正常 |
| `codex-execpolicy` | 执行策略（Starlark 规则）仍有意义 |
| `codex-protocol` | 协议层，平台无关 |
| `codex-api` / `codex-client` / `codex-http-client` | API 通信层，平台无关 |
| `codex-tools` | 工具抽象层，平台无关 |
| `codex-rollout` / `thread-store` / `state` | 持久化层（SQLite/JSONL），Proot 内正常 |
| `codex-extension-api` / `ext/*` | 扩展框架，平台无关 |
| `codex-prompts` | 提示词模板，平台无关 |
| `codex-skills` / `core-skills` | 技能系统，平台无关 |
| `memories/read` / `memories/write` | 记忆系统，平台无关 |
| `codex-otel` | OpenTelemetry 遥测，平台无关 |
| `codex-config` | 配置系统（需裁剪 TUI 部分，见下文） |
| `codex-core` | 核心引擎（需裁剪桌面沙箱/TUI 引用） |
| `codex-core-plugins` | 插件系统，平台无关 |
| `app-server-protocol` | 应用服务器协议/RPC，平台无关 |
| `agent-graph-store` | Agent 图存储，平台无关 |
| `model-provider` | 模型提供者，平台无关 |
| `message-history` | 消息历史，平台无关 |
| `codex-home` | Home 目录发现，Proot 内正常 |
| `features` | Feature Flag 系统，平台无关 |
| `utils/*`（全部） | 通用工具库，平台无关 |
| `rmcp-client` | MCP 客户端，平台无关 |
| `codex-login` | 认证模块（详见灰色地带结论） |
| `codex-secrets` | 密钥存储（详见灰色地带结论） |

---

## 三、确认需要裁剪的模块/功能

### 3.1 TUI 层（终端用户界面）

Android App 自带原生 UI，无终端环境。

| 裁剪目标 | 位置 | 说明 |
|---|---|---|
| `ratatui` 依赖 | 根 `Cargo.toml` L213-214 | TUI 渲染框架 |
| `ratatui-macros` 依赖 | 根 `Cargo.toml` L214 | TUI 宏 |
| `crossterm` 依赖 | 根 `Cargo.toml` L149, L397 | 终端控制（含 fork patch） |
| `tui_keymap.rs` | `config/src/tui_keymap.rs` | TUI 快捷键映射定义 |
| `Tui` 结构体 | `config/src/types.rs` L690-790 | TUI 配置（动画、主题、宠物、Vim 模式等） |
| `TuiNotificationSettings` | `config/src/types.rs` L662-676 | TUI 通知设置 |
| `AltScreenMode` | `protocol/src/config_types.rs` L584-604 | 终端备用屏幕模式 |
| `TUI_VISIBLE_COLLABORATION_MODES` | `protocol/src/config_types.rs` L633 | TUI 可见协作模式 |
| core config 中 ~20 个 `tui_*` 字段 | `core/src/config/mod.rs` L714-790 | TUI 通知/动画/Vim/状态栏/主题等 |
| `clippy.toml` TUI 颜色规则 | `clippy.toml` L9-14 | ratatui 颜色限制 |
| `DEFAULT_TERMINAL_RESIZE_REFLOW_FALLBACK_MAX_ROWS` | `config/src/types.rs` L687 | 终端 resize 相关 |

### 3.2 桌面沙箱（Landlock / Seccomp / Windows Sandbox）

Proot 自身即提供沙箱隔离，Android 内核不支持 Landlock/Seccomp BPF 用户态安装。

| 裁剪目标 | 位置 | 说明 |
|---|---|---|
| `landlock` 依赖 | `protocol/Cargo.toml` L50 | Linux 桌面 Landlock LSM |
| `seccompiler` 依赖 | `protocol/Cargo.toml` L51 | Linux Seccomp BPF 过滤器 |
| `SeccompInstall` / `SeccompBackend` 错误 | `protocol/src/error.rs` L43-51 | Seccomp 错误变体 |
| `LandlockRestrict` 错误 | `protocol/src/error.rs` L62-63 | Landlock 错误变体 |
| `WindowsSandboxLevel` | `protocol/src/config_types.rs`、`file-system/src/lib.rs` | Windows 沙箱级别 |
| `windows_sandbox_*` 字段 | `core/src/exec.rs`、`core/src/tools/sandboxing.rs` 等 | Windows 沙箱参数 |
| `codex_linux_sandbox_exe` 配置 | `core/src/config/mod.rs` | Linux 沙箱可执行文件路径 |
| `use_legacy_landlock` | `file-system/src/lib.rs` L298、sandboxing 相关 | 旧版 Landlock 开关 |
| `codex-sandboxing` 外部 crate | `core/Cargo.toml` L57 | 桌面沙箱封装（SandboxType 等） |
| `SandboxType::LinuxSeccomp` 相关逻辑 | `core/src/exec_tests.rs`、`core/src/tools/sandboxing.rs` | Seccomp 沙箱检测 |

### 3.3 平台专属代码（macOS / Windows）

目标平台为 Android（Linux 用户态 via Proot），非 macOS/Windows。

| 裁剪目标 | 位置 | 说明 |
|---|---|---|
| `core-foundation` 依赖 | `config/Cargo.toml` L53 | macOS Core Foundation |
| `windows-sys` 依赖 | `config/Cargo.toml` L56-61 | Windows 系统 API |
| `winapi-util` 依赖 | `config/Cargo.toml` L56、根 Cargo.toml L305 | Windows API 工具 |
| `uds_windows` 依赖 | 根 `Cargo.toml` L293 | Windows Unix Domain Socket |
| `system-configuration` 依赖 | 根 `Cargo.toml` L263 | macOS 系统配置（网络代理检测） |
| `#[cfg(target_os = "macos")]` 代码块 | 各 crate | macOS 条件编译 |
| `#[cfg(target_os = "windows")]` / `#[cfg(windows)]` 代码块 | 各 crate | Windows 条件编译 |
| `is_sandboxed()` seatbelt 检测 | `login/src/auth/default_client.rs` L384-386 | macOS sandbox 检测 |

### 3.4 桌面通知

| 裁剪目标 | 位置 | 说明 |
|---|---|---|
| `notify` 配置字段 | `config/src/config_toml.rs` L209-211 | 外部通知命令（notify-send） |
| `notify` 配置解析 | `config/src/loader/mod.rs` L68 | 通知配置加载 |
| core config `notify` 字段 | `core/src/config/mod.rs` L720 | 通知命令向量 |

### 3.5 桌面 Keyring（部分裁剪）

| 裁剪目标 | 位置 | 说明 |
|---|---|---|
| `keyring` crate 依赖 | 根 `Cargo.toml` L183 | 系统密钥环（仅 dev-dep in login） |
| `keyring` in secrets | `secrets/Cargo.toml` L24 | 密钥环存储后端 |

> **注意**：`secrets` 模块的 `KeyringStore` trait 已有 `DefaultKeyringStore`（no-op 实现），实际运行时可不依赖系统 keyring。`login` 中 keyring 仅为 dev-dependency。详见灰色地带结论。

---

## 四、灰色地带调研结论

### 4.1 `codex-login`（认证模块）

**结论：保留，无需删除**

调研发现：
- **无浏览器 OAuth 流**：`login/src/` 中未使用 `tiny_http` 启动本地回调服务器，也无 `webbrowser`/`open_url` 调用
- **多种认证方式**：`CodexAuth` 枚举支持 ApiKey、ChatGPT OAuth Tokens、AgentIdentity、PersonalAccessToken、Headers
- **Token 刷新为 HTTP 调用**：OAuth token 刷新通过 `/oauth/token` HTTP 端点完成，无需浏览器
- **已适配 Android**：`user_agent()` 已返回 `"codex-android/1.0"`
- `tiny_http` 虽在 `Cargo.toml` 中列为依赖，但 src 中未实际使用（可能为历史残留）

**待处理**：
- `tiny_http` 依赖可安全移除（未使用）
- `is_sandboxed()` 中 `CODEX_SANDBOX=seatbelt` 检测为 macOS 专属，需清理

### 4.2 `codex-secrets`（密钥存储）

**结论：保留，无需删除**

调研发现：
- **核心存储为文件加密**：使用 `age`（scrypt）加密的本地文件（`local.age`、`codex_auth.age`、`mcp_oauth.age`），在 Proot 文件系统内完全可用
- **Keyring 已抽象化**：`KeyringStore` trait 有 `DefaultKeyringStore`（no-op），`SecretsManager::new()` 默认使用 no-op keyring
- **keyring crate 非运行时必需**：在 `login` 中仅为 dev-dependency；`secrets` 中虽为 dependency 但通过 trait 抽象可绕过
- 三个命名空间：ManagedSecrets、CodexAuth、McpOAuth 均为文件存储

**待处理**：
- 若 Android 端需真正安全存储，后续可对接 Android Keystore（属重构范畴，非裁剪）
- `keyring` crate 依赖可考虑移除或 feature-gate

### 4.3 `codex-analytics`（分析遥测）

**结论：保留（外部 crate，平台无关）**

调研发现：
- 为外部 crate（不在本 workspace 内），提供 `AnalyticsEventsClient`、事件追踪
- 被 `core`、`core-skills`、`core-plugins` 广泛使用
- 提供：AppInvocation、SkillInvocation、CompactionEvent、TurnResolvedConfigFact 等
- 属于业务遥测抽象层，不依赖桌面平台特性
- 若 Android 端不需要遥测，可后续通过配置禁用，无需物理删除

### 4.4 `codex-connectors`（连接器/App 管理）

**结论：保留（外部 crate，平台无关）**

调研发现：
- 为外部 crate，提供 App/Plugin 连接器管理
- 提供：ConnectorRuntimeManager、ConnectorSnapshot、AppToolPolicy、AppMetadata 等
- 被 `core`、`tools`、`codex-mcp`、`ext/mcp`、`core-plugins` 使用
- 属于插件/连接器业务逻辑，不依赖桌面平台特性
- Codex Apps（MCP 服务器）在 Proot 内可作为子进程运行

### 4.5 Network Proxy（网络代理）

**结论：已删除，无需额外操作**

调研发现：
- `codex_network_proxy` crate **已被物理删除**（标记为 REMOVED-DELETED-CRATE）
- 残留引用仅为注释，不影响编译
- `core/src/config/` 中 `NetworkProxySpec` 仍存在，用于沙箱内网络访问控制
- 在 Proot 环境下，网络策略控制仍有意义（限制子进程网络访问）
- 现有 `NetworkProxySpec` 为配置/策略层，不依赖已删除的 proxy 实现

---

## 五、裁剪优先级建议

### P0 - 必须裁剪（编译阻断/平台不兼容）

1. **Landlock / Seccomp**（`protocol` crate）— Android 内核不支持
2. **Windows / macOS 平台依赖**（`config` crate）— 目标平台不存在
3. **`codex-sandboxing` 外部 crate 引用**（`core`）— 桌面沙箱实现

### P1 - 应当裁剪（死代码/无用功能）

4. **TUI 配置**（`config/src/tui_keymap.rs`、`types.rs` Tui 结构体）
5. **TUI 依赖**（ratatui、crossterm、ratatui-macros）
6. **桌面通知**（notify 配置）
7. **AltScreenMode / TUI_VISIBLE_COLLABORATION_MODES**
8. **`tiny_http` 未使用依赖**（login）
9. **`clippy.toml` TUI 颜色规则**

### P2 - 可选裁剪（代码整洁）

10. **REMOVED-DELETED-CRATE 注释清理**
11. **`keyring` crate 依赖移除/feature-gate**
12. **`is_sandboxed()` seatbelt 检测清理**
13. **`os_info` 依赖评估**（login 中用于 User-Agent，可保留）

---

## 六、外部 Crate 依赖状态（不在 workspace 内，被引用）

| 外部 Crate | 使用方 | 状态 | 说明 |
|---|---|---|---|
| `codex-sandboxing` | core | **待裁剪** | 桌面沙箱类型定义 |
| `codex-analytics` | core, core-skills, core-plugins | 保留 | 业务遥测 |
| `codex-connectors` | core, tools, codex-mcp, ext/mcp, core-plugins | 保留 | 连接器管理 |
| `codex-execpolicy` | core, config | 保留 | 执行策略引擎 |
| `codex-plugin` | core, ext/mcp, core-plugins, codex-mcp | 保留 | 插件系统 |
| `codex-model-provider-info` | core, config, login, model-provider, memories/write | 保留 | 模型信息 |
| `codex-models-manager` | core, model-provider, memories/write | 保留 | 模型管理 |
| `codex_exec_server` | — | **已删除** | 多进程执行服务器 |
| `codex_network_proxy` | — | **已删除** | 网络代理实现 |

---

## 七、注意事项

1. **Proot 环境假设**：所有"保留"决策基于 Proot 提供完整 Linux 用户态（bash、git、文件系统、子进程、PTY）
2. **非重构原则**：当前仅做物理删除，不做功能适配/重写
3. **编译验证**：每次裁剪后需确保 workspace 可编译通过
4. **外部 crate 处理**：`codex-sandboxing` 等外部 crate 若无法直接删除，可考虑 stub/feature-gate
5. **向后兼容**：保留的 `REMOVED-DELETED-CRATE` 注释不影响编译，清理为 P2 优先级

---

---

## 八、平台关键词搜索结果（详细定位）

### 8.1 Windows 专属代码分布

#### `core` crate

| 文件 | 内容 | 说明 |
|---|---|---|
| `core/src/config/mod.rs` L6-8 | `use crate::windows_sandbox::*` | Windows 沙箱模块（**文件已不存在，引用残留**） |
| `core/src/config/mod.rs` L339-361 | `windows_sandbox_mode`、`windows_sandbox_private_desktop` 字段 | Config 中 Windows 沙箱配置 |
| `core/src/config/mod.rs` L3070-3191 | `resolve_windows_sandbox_mode` 逻辑 | Windows 沙箱模式解析 |
| `core/src/tools/runtimes/mod.rs` L172-531 | `disable_powershell_profile_for_elevated_windows_sandbox()` + 测试 | PowerShell 沙箱处理 |
| `core/src/exec.rs` L43-51 | `WindowsSandboxFilesystemOverrides`、`windows_sandbox_uses_elevated_backend` 等 | Windows 沙箱文件系统覆盖 |
| `core/src/exec.rs` L512 | `#[cfg_attr(not(windows), allow(unused_variables))]` | Windows 条件编译 |
| `core/src/tools/runtimes/unified_exec.rs` L406-413 | PowerShell UTF-8 前缀、Windows 沙箱 profile | Windows shell 处理 |
| `core/src/tools/runtimes/shell.rs` L25,45,298 | PowerShell 相关 import 和逻辑 | Windows shell 处理 |
| `core/src/exec_policy_tests.rs` L40 | `mod windows_tests` | Windows 执行策略测试 |
| `core/src/session/mod.rs` L331 | `use crate::windows_sandbox::WindowsSandboxLevelExt` | Windows 沙箱引用 |

#### `shell-command` crate

| 文件 | 内容 | 说明 |
|---|---|---|
| `command_safety/mod.rs` L1 | `mod powershell_parser` | **文件已不存在（powershell_parser.rs 缺失）** |
| `command_safety/mod.rs` L6 | `pub(crate) mod windows_safe_commands` | **文件已不存在（windows_safe_commands.rs 缺失）** |
| `parse_command.rs` L4,17 | `use crate::powershell::extract_powershell_command` | **powershell 模块不存在** |
| `is_safe_command.rs` L41,597-611 | `is_safe_powershell_words()`、PowerShell 路径查找 | PowerShell 安全命令检测 |
| `is_dangerous_command.rs` L46,309-318 | `dangerous_powershell_words_match()` | PowerShell 危险命令检测 |
| `shell_detect.rs` L245-296 | PowerShell/Cmd shell 类型检测 | Windows shell 检测 |

#### `config` crate

| 文件 | 内容 | 说明 |
|---|---|---|
| `config/src/types.rs` L158-166 | `WindowsSandboxModeToml`、`WindowsToml` 结构体 | Windows 配置类型 |
| `config/src/config_toml.rs` L29,499-501,747 | `windows: Option<WindowsToml>`、`windows_sandbox_level` | Windows TOML 配置 |
| `config/src/hook_config.rs` L153-154 | `command_windows` / `commandWindows` 字段 | Hook Windows 命令 |
| `config/src/hook_config.rs` L199-204 | `#[cfg(windows)]` windows_managed_dir | Windows 管理目录 |
| `config/src/loader/mod.rs` L48-55,636-693 | `#[cfg(windows)]` 系统配置路径（ProgramData） | Windows 系统级配置 |
| `config/src/loader/mod.rs` L1068,1431,1532,1554 | `cfg!(windows)` 条件逻辑 | Windows 平台判断 |
| `config/src/profile_toml.rs` L11,64 | `windows: Option<WindowsToml>` | Profile 中 Windows 配置 |

#### `protocol` crate

| 文件 | 内容 | 说明 |
|---|---|---|
| `protocol/src/config_types.rs` | `WindowsSandboxLevel` 枚举 | Windows 沙箱级别定义 |
| `protocol/src/protocol.rs` L30 | `use crate::config_types::WindowsSandboxLevel` | 协议中引用 |
| `protocol/src/shell_environment.rs` L100 | `mod windows_tests` | Windows shell 环境测试 |

#### `file-system` crate

| 文件 | 内容 | 说明 |
|---|---|---|
| `file-system/src/lib.rs` L294-296 | `windows_sandbox_level`、`windows_sandbox_private_desktop` 字段 | 文件系统沙箱上下文 |

#### `utils/absolute-path`

| 文件 | 内容 | 说明 |
|---|---|---|
| `utils/absolute-path/src/lib.rs` L36,218,429,632,648 | `cfg!(windows)`、`#[cfg(target_os = "windows")]` | Windows 路径处理 |

---

### 8.2 macOS 专属代码分布

| 文件 | 内容 | 说明 |
|---|---|---|
| `core/src/sandboxing/mod.rs` L14-15 | `#[cfg(target_os = "macos")] use crate::spawn::CODEX_SANDBOX_ENV_VAR` | macOS seatbelt 沙箱 |
| `core/src/tools/runtimes/mod.rs` L15-16,335 | `#[cfg(target_os = "macos")]` | macOS 运行时逻辑 |
| `core/tests/suite/request_permissions.rs` L624,727,1812 | `#[cfg(target_os = "macos")]` 测试 | macOS 权限测试 |
| `core/tests/suite/unified_exec.rs` L3212 | `#[cfg(target_os = "macos")]` | macOS 执行测试 |
| `core/src/config/config_loader_tests.rs` L992-1352 | 6 处 `#[cfg(target_os = "macos")]` | macOS 配置加载测试 |
| `config/src/loader/mod.rs` L156-167 | macOS managed preferences (MDM) 加载 | macOS 企业设备管理 |
| `config/src/loader/layer_io.rs` L71-72 | `#[cfg(target_os = "macos")]` managed admin layer | macOS MDM 配置层 |
| `config/src/state.rs` L53-75 | `managed_preferences_base64`、`macos_managed_config_requirements_base64` | macOS 托管偏好 |
| `login/src/auth/default_client.rs` L384-386 | `CODEX_SANDBOX=seatbelt` 检测 | macOS 沙箱检测 |
| `login/src/auth/default_client_tests.rs` L281 | `#[cfg(target_os = "macos")]` | macOS 测试 |
| `http-client/src/custom_ca.rs` L25-28 | seatbelt 相关注释 | macOS 沙箱说明 |
| `config/Cargo.toml` L52-53 | `core-foundation = "0.9"` | macOS Core Foundation 依赖 |

---

### 8.3 Linux 桌面沙箱代码（Android 需移除）

| 文件 | 内容 | 说明 |
|---|---|---|
| `protocol/src/error.rs` L44-51 | `#[cfg(target_os = "linux")]` Seccomp 错误 | Seccomp 错误变体 |
| `protocol/src/error.rs` L62-63 | `LandlockRestrict` 错误 | Landlock 错误变体 |
| `protocol/src/error.rs` L156,159,206 | `#[cfg(target_os = "linux")]` | Linux 沙箱相关 |
| `protocol/Cargo.toml` L49-51 | `landlock`、`seccompiler` 依赖 | Linux 桌面沙箱依赖 |
| `core/src/exec.rs` L300-310,324-325,394-395 | `codex_linux_sandbox_exe`、`use_legacy_landlock` | Linux 沙箱执行 |
| `core/src/config/mod.rs` L919-924 | `codex_linux_sandbox_exe` 配置字段 | Linux 沙箱 exe 路径 |
| `core/src/session/turn_context.rs` L356 | `use_legacy_landlock` | Landlock 开关 |
| `core/src/mcp_tool_call.rs` L744-746 | `codex_linux_sandbox_exe`、`use_legacy_landlock` | MCP 中沙箱引用 |
| `core/src/exec_tests.rs` L29-103 | `SandboxType::LinuxSeccomp` 检测测试 | Seccomp 测试 |

> **注意**：`#[cfg(target_os = "linux")]` 代码并非全部需要移除。Proot 运行在 Linux 内核上，通用 Linux 代码（如路径处理、信号处理）应保留。仅 **Landlock/Seccomp 桌面沙箱** 相关需移除。

---

### 8.4 已发现的编译断裂（缺失文件）

以下模块在代码中被声明/引用，但对应源文件已不存在：

| 声明位置 | 缺失模块 | 影响 |
|---|---|---|
| `shell-command/src/command_safety/mod.rs` L1 | `powershell_parser.rs` | shell-command 编译失败 |
| `shell-command/src/command_safety/mod.rs` L6 | `windows_safe_commands.rs` | shell-command 编译失败 |
| `shell-command/src/parse_command.rs` L4 | `crate::powershell` 模块 | shell-command 编译失败 |
| `core/src/config/mod.rs` L6-8 | `crate::windows_sandbox` 模块 | core 编译失败 |
| `core/src/session/mod.rs` L331 | `crate::windows_sandbox::WindowsSandboxLevelExt` | core 编译失败 |
| `core/src/sandboxing/mod.rs` L15-16 | `crate::spawn::CODEX_SANDBOX_ENV_VAR` / `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` | core 编译失败（`spawn` 模块不存在于 core 根） |
| `core-plugins/src/startup_sync_tests.rs` L732 | `macos_git_binary_from_path()` 函数 | 测试编译失败（函数已从 startup_sync.rs 移除） |

> 这表明裁剪工作已部分进行但**尚未完成**，当前代码库处于不可编译状态。

---

### 8.5 `#[cfg(unix)]` 代码（保留）

Android 基于 Linux 内核，属于 `unix` 目标。所有 `#[cfg(unix)]` 代码在 Proot 内有效，**应保留**：
- `core/src/tools/runtimes/` 中的 unix 信号处理、进程管理
- `core-plugins/src/startup_sync.rs` 中的 unix 权限
- `login/src/auth/storage.rs` 中的 unix 文件权限（`OpenOptionsExt`）
- 各 crate 中的 unix 条件编译

---

## 九、PowerShell / Cmd 相关代码（Windows 专属 Shell）

PowerShell 和 Cmd 是 Windows 专属 shell，在 Android Proot 环境中不存在。

| 位置 | 内容 | 处理建议 |
|---|---|---|
| `shell-command/src/shell_detect.rs` | PowerShell/Cmd 类型检测 | 裁剪 |
| `shell-command/src/command_safety/is_safe_command.rs` | `is_safe_powershell_words()` | 裁剪 |
| `shell-command/src/command_safety/is_dangerous_command.rs` | `dangerous_powershell_words_match()` | 裁剪 |
| `core/src/shell.rs` L31,41,65,67 | `ShellType::PowerShell`、`ShellType::Cmd` 分支 | 裁剪 |
| `core/src/tools/mod.rs` L56,58 | PowerShell/Cmd 映射 | 裁剪 |
| `core/src/tools/runtimes/unified_exec.rs` L406-413 | PowerShell UTF-8 前缀 | 裁剪 |
| `core/src/tools/runtimes/shell.rs` L298 | PowerShell 命令处理 | 裁剪 |
| `core/src/exec_policy.rs` L835 | `parse_powershell_command_into_plain_commands` | 裁剪 |
| `codex-tools` `ToolUserShellType::PowerShell/Cmd` | Shell 类型枚举 | 裁剪 |

> **注意**：`ShellType` 枚举本身应保留（Bash/Zsh/Fish 在 Proot 内可用），仅移除 PowerShell/Cmd 变体。

---

## 十、补充搜索：桌面 IDE 集成 / 沙箱体系 / 其他平台代码


### 10.2 `codex_sandboxing` 外部 crate 深度引用（核心裁剪目标）

该 crate 提供桌面沙箱完整实现，在 `core` 中被**深度耦合**：

| 引用文件 | 使用的类型/函数 |
|---|---|
| `core/src/exec.rs` L38-51 | `SandboxCommand`, `SandboxManager`, `SandboxTransformRequest`, `SandboxType`, `SandboxablePreference`, `WindowsSandboxFilesystemOverrides`, `is_likely_sandbox_denied`, `permission_profile_supports_windows_restricted_token_sandbox`, `resolve_windows_elevated_filesystem_overrides`, `resolve_windows_restricted_token_filesystem_overrides`, `unsupported_windows_restricted_token_sandbox_reason`, `windows_sandbox_uses_elevated_backend` |
| `core/src/config/mod.rs` L155-156 | `compatibility_sandbox_policy_for_permission_profile`, `system_bwrap_warning` |
| `core/src/tools/runtimes/mod.rs` L19-20 | `SandboxCommand`, `SandboxType` |
| `core/src/tools/runtimes/unified_exec.rs` L46-47 | `SandboxCommand`, `SandboxablePreference` |
| `core/src/tools/runtimes/shell.rs` L44 | `SandboxablePreference` |
| `core/src/unified_exec/process.rs` L19 | `SandboxType` |
| `core/src/unified_exec/process_manager.rs` L64 | `SandboxCommand` |
| `core/src/session/turn_context.rs` L16-18 | `compatibility_sandbox_policy_for_permission_profile`, `effective_file_system_sandbox_policy`, `effective_network_sandbox_policy` |
| `core/src/session/mod.rs` L145 | `intersect_permission_profiles` |
| `core/src/session/tests.rs` L1084-1085 | `SandboxablePreference` |
| `core/src/exec_tests.rs` L425-968 | `SandboxType::WindowsRestrictedToken`（15 处） |
| `core/src/tools/runtimes/mod.rs` L179-531 | `SandboxType::WindowsRestrictedToken`（6 处） |

> **裁剪策略**：`codex_sandboxing` 是核心裁剪难点。需要：
> 1. 移除所有 Windows 沙箱类型（`WindowsRestrictedToken`, `WindowsElevated`）
> 2. 移除 Linux 桌面沙箱（`LinuxSeccomp`, `LinuxLandlock`, bwrap）
> 3. 保留 `SandboxType::None` 作为唯一选项（Proot 自身即沙箱）
> 4. 可能需要 stub 整个 crate 或 feature-gate

### 10.3 macOS Seatbelt 沙箱

| 位置 | 内容 | 说明 |
|---|---|---|
| `core/src/sandboxing/mod.rs` L14-15 | `use crate::spawn::CODEX_SANDBOX_ENV_VAR` | **引用已不存在的 `spawn` 模块** |
| `core/tests/suite/exec.rs` L19-20,130 | seatbelt 环境变量检测、`openpty_works_under_real_exec_seatbelt_path` | macOS 沙箱测试 |
| `core/tests/suite/unified_exec.rs` L3214-3332 | `unified_exec_python_prompt_under_seatbelt` | macOS 沙箱测试 |
| `core/tests/common/lib.rs` L539-553 | seatbelt 跳过逻辑 | macOS 沙箱测试辅助 |
| `login/src/auth/default_client.rs` L385 | `CODEX_SANDBOX=seatbelt` 检测 | macOS 沙箱判断 |
| `http-client/src/custom_ca.rs` L25-28 | seatbelt 注释说明 | macOS 沙箱文档 |

### 10.4 macOS 企业设备管理（MDM / Managed Preferences）

| 位置 | 内容 | 说明 |
|---|---|---|
| `config/src/loader/mod.rs` L156-176 | `macos::load_managed_admin_requirements_layer()` | macOS MDM 配置加载 |
| `config/src/loader/layer_io.rs` L71-72 | `map_managed_admin_layer()` | macOS 管理员配置映射 |
| `config/src/state.rs` L52-75 | `macos_managed_config_requirements_base64` 字段 | macOS 托管配置状态 |
| `config/src/loader/mod.rs` L86-98 | `%ProgramData%\OpenAI\Codex\requirements.toml` (Windows) | Windows 企业配置路径 |

### 10.5 Windows 系统 API 调用

| 位置 | 内容 | 说明 |
|---|---|---|
| `config/src/host_name.rs` L6-9,57-63 | `winapi_util::sysinfo::get_computer_name` | Windows 主机名获取 |
| `config/src/loader/mod.rs` L697-727 | `windows_sys::Win32::UI::Shell::SHGetKnownFolderPath` | Windows ProgramData 路径 |
| `config/src/loader/mod.rs` L56 | `DEFAULT_PROGRAM_DATA_DIR_WINDOWS` | Windows 默认路径常量 |
| `protocol/src/shell_environment.rs` L117-138 | `AppData` 环境变量测试 | Windows 环境测试 |

### 10.6 Bubblewrap（Linux 桌面沙箱）

| 位置 | 内容 | 说明 |
|---|---|---|
| `features/src/lib.rs` L129,264,992 | `use_linux_sandbox_bwrap` feature flag（已标记为 no-op） | 旧版 bwrap 开关 |
| `core/src/config/mod.rs` L156 | `pub use codex_sandboxing::system_bwrap_warning` | bwrap 系统警告 |
| `protocol/src/permissions.rs` L1036 | bwrap 路径映射注释 | 沙箱路径说明 |

### 10.7 其他已发现的编译断裂

| 声明位置 | 缺失模块/函数 | 影响 |
|---|---|---|
| `core/src/sandboxing/mod.rs` L15-16 | `crate::spawn::CODEX_SANDBOX_ENV_VAR` / `CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR` | core 编译失败（`spawn` 模块不存在于 core 根） |
| `core-plugins/src/startup_sync_tests.rs` L732 | `macos_git_binary_from_path()` 函数 | 测试编译失败（函数已从 startup_sync.rs 移除） |

### 10.8 `#[cfg(unix)]` 代码（保留，Android 适用）

以下 unix 代码在 Proot 内有效，**应保留**：
- `config/src/host_name.rs` L1-4,41-55：`dns_lookup`/`getaddrinfo` DNS 解析
- `core/src/tools/runtimes/` 中的 unix 信号处理、进程管理
- `core-plugins/src/startup_sync.rs` 中的 unix 文件权限
- `login/src/auth/storage.rs` 中的 unix 文件权限（`OpenOptionsExt`）

---

## 十一、裁剪影响范围统计

| 类别 | 涉及 crate 数 | 涉及文件数（估） | 复杂度 |
|---|---|---|---|
| `codex_sandboxing` 外部 crate 解耦 | 1（core） | ~12 文件 | **高**（深度耦合） |
| Windows 沙箱 / 平台代码 | 5（core, config, protocol, file-system, shell-command） | ~20 文件 | 中 |
| macOS 沙箱 / MDM / 平台代码 | 4（core, config, login, http-client） | ~10 文件 | 中 |
| TUI 层 | 3（config, protocol, core） | ~6 文件 | 低 |
| PowerShell / Cmd Shell | 2（shell-command, core） | ~8 文件 | 中 |
| 桌面 IDE 集成 | 4（config, core, protocol, app-server-protocol） | ~6 文件 | 低 |
| 桌面通知 / Keyring | 2（config, secrets） | ~4 文件 | 低 |

---

*文档创建时间：2026-07-23*  
*最后更新：2026-07-23（新增第十、十一节：桌面 IDE 集成、codex_sandboxing 深度引用、macOS seatbelt/MDM、Windows API、bubblewrap、编译断裂补充、裁剪影响统计）*
