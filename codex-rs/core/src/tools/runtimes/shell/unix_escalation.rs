use std::path::Path;
use anyhow::Result;
use codex_protocol::protocol::Op;
use codex_sandboxing::SandboxManager;
use crate::tools::runtimes::shell::ShellResult;

pub async fn run_unix_escalation(
    _sandbox_manager: &SandboxManager,
    _cwd: &Path,
    _op: &Op,
) -> Result<ShellResult> {
    anyhow::bail!("unix shell escalation is disabled on mobile/embedded agent")
}
