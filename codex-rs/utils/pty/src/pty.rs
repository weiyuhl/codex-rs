use anyhow::Result;
use crate::process::ProcessHandle;
use crate::process::SpawnedProcess;
use crate::process::TerminalSize;
use std::collections::HashMap;
use std::path::Path;

/// Report whether ConPTY is available on this platform.
pub fn conpty_supported() -> bool {
    false
}

/// Fallback PTY spawn helper (delegates to pipe process for mobile/embedded).
pub fn spawn_process(
    program: &Path,
    args: &[impl AsRef<std::ffi::OsStr>],
    cwd: &Path,
    env: &HashMap<String, String>,
    size: Option<TerminalSize>,
) -> Result<SpawnedProcess> {
    let _ = size;
    crate::pipe::spawn_process(program, args, cwd, env)
}
