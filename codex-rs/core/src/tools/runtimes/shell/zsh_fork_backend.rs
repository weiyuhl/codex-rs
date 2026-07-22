use std::path::Path;
use anyhow::Result;
use codex_protocol::protocol::Op;
use codex_sandboxing::SandboxManager;
use crate::tools::runtimes::shell::ShellResult;

pub async fn run_zsh_fork_backend(
    _sandbox_manager: &SandboxManager,
    _cwd: &Path,
    _op: &Op,
) -> Result<ShellResult> {
    anyhow::bail!("zsh fork backend is disabled on mobile/embedded agent")
}
