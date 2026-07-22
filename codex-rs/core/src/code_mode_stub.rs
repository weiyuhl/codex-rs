use std::path::PathBuf;

pub trait CodeModeSessionProvider: Send + Sync {}

#[derive(Debug, Clone, Default)]
pub struct InProcessCodeModeSessionProvider;

impl CodeModeSessionProvider for InProcessCodeModeSessionProvider {}

#[derive(Debug, Clone)]
pub struct ProcessOwnedCodeModeSessionProvider {
    _host_program: PathBuf,
}

impl ProcessOwnedCodeModeSessionProvider {
    pub fn new(host_program: PathBuf) -> Self {
        Self {
            _host_program: host_program,
        }
    }
}

impl CodeModeSessionProvider for ProcessOwnedCodeModeSessionProvider {}

pub fn is_code_mode_nested_tool(_tool_name: &str) -> bool {
    false
}

pub const DEFAULT_EXEC_YIELD_TIME_MS: u64 = 1000;
pub const PUBLIC_TOOL_NAME: &str = "code_mode";
pub const WAIT_TOOL_NAME: &str = "code_mode_wait";

#[derive(Debug, Clone, Default)]
pub struct ToolNamespaceDescription {
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolDefinition {
    pub name: String,
}
