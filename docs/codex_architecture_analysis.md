# Codex 项目与 `codex-rs` 深度架构分析报告

> **文档状态**：已归档  
> **更新时间**：2026-07-22  
> **工作区路径**：`d:/codex-main`  
> **修改记录**：  
> 1. `codex-rs/tui` 模块已于 2026-07-22 成功从中移除，Workspace 依赖项已同步完成清理。  
> 2. `codex-rs/app-server` 及 `codex-rs/app-server-client` 模块已于 2026-07-22 成功从中移除，Workspace 依赖项已同步完成清理。  
> 3. 清理非 Rust 构建链文件（Bazel、SDK、Node/pnpm）。  
> 4. `codex-rs/cli` 桌面命令行二进制模块已于 2026-07-22 成功从中移除，Workspace 依赖项已同步完成清理。  
> 5. 实施方案 B：彻底从源码层面剥离 Windows 及 macOS 平台依赖，重构沙箱路由为纯 Linux / Android 模式。  
> 6. 彻底剥离官方账号登录授权（删除 OAuth 2.0 PKCE 本地 Web 服务器 `LoginServer`、Device Code 设备码服务及 HTML 回调网页）。  
> 7. 彻底剥离 AWS Bedrock 凭据与 `aws-auth` 签名 Crate 及外部 AWS 依赖。  
> 8. 彻底剥离云端服务模块（删除 `cloud-config`、`cloud-tasks`、`cloud-tasks-client`、`cloud-tasks-mock-client`）。  
> 9. 彻底剥离外部 Agent 迁移模块（删除 `external-agent-migration`）。  
> 10. 彻底剥离 4 大桌面端冗余模块（删除 `feedback` 桌面反馈、`install-context` 包管理器检测、`stdio-to-uds` 管道重定向、`utils/sleep-inhibitor` 防休眠锁）。  
> 11. 彻底剥离 V8 引擎 POC 模块（删除 `v8-poc`）。  
> 12. 彻底剥离桌面 Linux Bubblewrap 隔离包装模块（删除 `bwrap`）。  
> 13. 彻底剥离 app-server 残留守护进程/Socket传输/测试模块（删除 `app-server-daemon`、`app-server-transport`、`app-server-test-client`，保留 `app-server-protocol` Schema 数据定义库）。  
> 14. 彻底剥离 5 大 Android 平台冗余与不可用模块（删除 `terminal-detection` 桌面终端检测、`keyring-store` 桌面系统凭据锁、`utils/cli` 桌面命令行辅助、`thread-manager-sample` 样例代码，精简重构 `utils/pty` 虚拟终端控制）。  
> 15. 彻底剥离第二批 5 大 Android 冗余/不可用/悬空模块（删除 `shell-escalation` 桌面提权、`uds` 悬空 Socket 库、`test-binary-support` 测试 Mock 库、`utils/approval-presets` 桌面 CLI 预设、`utils/cargo-bin` 桌面 Cargo 查找器）。  
> 16. 彻底剥离第三批 3 大 Android 冗余/不可用模块（删除 `linux-sandbox` 桌面 Linux 沙箱镜像、`file-watcher` 悬空监听库、`websocket-client` 桌面 WebSocket 传输层）。  
> 17. 彻底剥离第四批 3 大 Android 冗余/悬空/空壳模块（删除 `utils/readiness` 悬空就绪检测库、`utils/sandbox-summary` 桌面 CLI 彩色报告库，合并内联 `collaboration-mode-templates` 空壳库）。  
> 18. 彻底剥离第五批 4 大 Android 冗余/桌面模型管理器模块（删除 `ext/guardian` 悬空审查扩展、`lmstudio` 桌面本地大模型连接器、`ollama` 桌面本地大模型连接器、`utils/oss` 本地模型包装库）。  
> 19. 彻底剥离第六批 3 大 Android 冗余/悬空/过度拆分模块（删除 `ext/connectors` 悬空连接器扩展、`utils/fuzzy-match` 桌面终端补全库，合并内联 `async-utils` 到 `core`）。  
> 20. 彻底剥离第七批 2 大 Android 冗余/悬空/过度拆分模块（删除 `utils/elapsed` 悬空耗时格式化库，合并内联 `utils/json-to-toml` 到 `mcp-server`）。  
> 21. 彻底剥离第八批 3 大 Android 冗余/悬空/过度拆分模块（删除 `core-api` 悬空接口库、`ext/agent` 悬空扩展库，合并内联 `context-fragments` 到 `core`）。  
> 22. 彻底剥离第九批 1 大 Android 冗余桌面路径查找库（物理删除 `utils/home-dir` 桌面 Home 路径检测库，内联 `find_codex_home` 到 `codex-home`）。  
> 23. 彻底剥离第十批 1 大 Android 冗余报文 Debug 库（物理删除 `response-debug-context` 报文 Debug 解析库，合并内联到 `codex-api`）。  
> 24. 彻底剥离第十一批 4 大 Android 严重不兼容/不可用中大型模块（删除 `exec-server` 守护服务器、`network-proxy` MITM CA 代理、`mcp-server` Stdio MCP 服务端、`code-mode-host` Node.js 宿主，共 57,000+ 行代码）。  
> 25. 彻底剥离第十二批 4 大 Android 严重不兼容/桌面控制台模块（物理删除 `sandboxing` 桌面系统沙箱包装器、`exec` CLI 命令行二进制入口、`arg0` 命令行符号链接路由分发器、`execpolicy` 桌面 PowerShell/Brew 安全策略规则库，共 16,338 行代码）。  
> 26. `codex-rs/` 全量代码统计数据完成归档。  
> 26. 彻底剥离 Windows 沙箱及进程加固残余模块（删除 `windows-sandbox-rs`、`process-hardening`、`app-server-client` 引用）。  
> 27. 全面清理 152 个 Rust 源文件中残留引用（移除 `use codex_exec_server/network_proxy/...` 等，-539 行）。  
> 28. `codex-rs/` 目录树重建：基于实际 70 个 Crate 重写架构明细（1,587 文件 / 602,026 LoC）。  
> 29. 文档同步更新至 2026-07-22 最新状态。  

---

## 1. 项目概览与定位 (Project Overview)

**Codex** 是由 OpenAI 开发的高性能、跨平台 AI 编程 Agent（Coding Agent）生态系统。核心引擎由 Rust 打造，专为 **Android App 嵌入式 Agent 核心** 打造（原桌面端 CLI/GUI/守护进程模块已全部剥离）。

---

## 2. `codex-rs/` 目录树结构 (70 Workspace Crates)

```
d:/codex-main/codex-rs/    (70 workspace crates, 1,587 .rs files, 602,026 LoC)
├── 1. 核心 Agent 引擎层 (Core Engine & Orchestration)
│   ├── core/                 [核心] Agent 调度心脏 (ThreadManager, ContextManager, Prompt 压缩)
│   ├── core-plugins/         Core 内置插件管理器
│   ├── core-skills/          Core 内置 Skill 技能库解析
│   ├── agent-graph-store/    Agent 图结构持久化存储
│   ├── agent-identity/       Agent 身份标识管理
│   ├── codex-api/            OpenAI 官方 API 接口抽象
│   ├── codex-backend-openapi-models/  后端 OpenAPI 数据模型
│   ├── codex-home/           $CODEX_HOME 目录管理
│   ├── codex-client/         Codex 客户端 SDK
│   ├── config/               ConfigToml 解析与 Schema 生成
│   ├── protocol/             核心 Turn/Op 消息协议
│   ├── rollout/              发布部署逻辑
│   ├── rollout-trace/        发布链路追踪
│   ├── state/                主状态管理器
│   ├── thread-store/         对话线程持久化存储
│   └── prompts/              Prompt 模版仓库
│
├── 2. 通信协议与客户端 (Protocols & Transport)
│   ├── app-server-protocol/  [核心] JSON-RPC 2.0 协议 v1/v2
│   ├── code-mode/            代码模式运行逻辑
│   ├── code-mode-protocol/   Code Mode 通信协议
│   ├── exec-server-protocol/ 远端执行环境 RPC 协议
│   ├── responses-api-proxy/  API 代理服务
│   ├── rmcp-client/          Rust MCP 客户端连接库
│   ├── http-client/          HTTP 客户端抽象层
│   └── backend-client/       内部 Backend API 客户端
│
├── 3. 工具、技能与 MCP 系统 (Tools, Skills & MCP)
│   ├── codex-mcp/            [核心] MCP 连接管理器
│   ├── skills/               用户技能定义与加载
│   ├── plugin/               插件包声明与加载器
│   ├── tools/                内置 Tool 处理器注册表
│   ├── connectors/           外部服务连接器
│   ├── file-search/          高性能文件搜索
│   ├── file-system/          安全文件系统读写抽象
│   ├── shell-command/        Shell 命令封装
│   ├── apply-patch/          Git Patch 对比与文件替换
│   ├── git-utils/            Git 仓库状态感知
│   ├── ext/                  扩展架构
│   │   ├── extension-api/    Extension API 标准
│   │   ├── goal/             Goal 目标模式
│   │   ├── image-generation/ 图像生成
│   │   ├── items/            Item 数据条目
│   │   ├── mcp/              MCP 插件
│   │   ├── memories/         记忆系统
│   │   ├── skills/           Skill 插件
│   │   └── web-search/       Web 搜索
│   └── memories/
│       ├── read/             记忆读取
│       └── write/            记忆写入
│
├── 4. 安全沙箱与执行控制 (Sandboxing & Security)
│   ├── sandboxing/           [核心] 沙箱抽象接口
│   ├── execpolicy/           命令执行策略引擎
│   ├── exec/                 非交互式执行器
│   ├── secrets/              密钥敏感数据掩码
│   ├── login/                用户登录鉴权
│   ├── hooks/                钩子系统
│   └── codex-experimental-api-macros/  实验性 API 宏
│
├── 5. 模型 Providers 与网络层 (Model Providers)
│   ├── model-provider/       [核心] 模型 API Provider 抽象
│   ├── model-provider-info/  各 Provider 能力清单
│   ├── models-manager/       动态模型目录管理
│   ├── chatgpt/              ChatGPT 后端接口
│   ├── analytics/            使用量数据收集 (网络暂停)
│   └── otel/                 OpenTelemetry 追踪 (网络暂停)
│
├── 6. 运行时与基础设施 (Runtime & Infra)
│   ├── features/             Feature Toggles 判定
│   ├── arg0/                 进程可执行文件名解析
│   └── utils/
│       ├── absolute-path/    绝对路径安全转换
│       ├── path-uri/         路径 URI 转换
│       ├── path-utils/       路径规范化工具
│       ├── cache/            缓存工具
│       ├── image/            图片数据转换与缩放
│       ├── output-truncation/ 输出超长截断
│       ├── plugins/          插件辅助工具
│       ├── pty/              伪终端控制
│       ├── rustls-provider/  Rustls 加密提供者
│       ├── stream-parser/    流式数据解析器
│       ├── string/           字符串处理扩展
│       └── template/         文本模版渲染
```

---

## 3. 构建与开发常用命令 (Developer Cheat Sheet)

工作区使用 `justfile` 统筹构建流程：

| 命令 | 说明 |
| :--- | :--- |
| `just fmt` | 自动格式化 Rust、Python、JSON 等所有文件 |
| `just fix -p <crate>` | 针对指定 Crate 自动修复 Clippy 问题 |
| `just test -p <crate>` | 运行指定 Crate 的单元与集成测试 |
| `just write-config-schema` | 重新生成并更新 `config.schema.json` |

---

## 4. 循环依赖已知问题

`core → core-plugins → core-skills → core` 存在循环依赖，`core-skills` 从 `core` 导入 `ContextualUserFragment` trait，待后续解耦至共享 crate。
