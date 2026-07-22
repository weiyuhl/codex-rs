use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::models::PermissionProfile;
use codex_utils_absolute_path::AbsolutePathBuf;
use codex_utils_pty::SpawnedProcess;
use codex_utils_pty::TerminalSize;

use crate::SandboxType;
use crate::WindowsSandboxFilesystemOverrides;
use crate::WindowsSandboxProxySettingsMode;

/// Windows-specific inputs for an executor-native process spawn.
pub struct WindowsSandboxSpawnRequest<'a> {
    pub permission_profile: &'a PermissionProfile,
    pub workspace_roots: &'a [AbsolutePathBuf],
    pub windows_sandbox_level: WindowsSandboxLevel,
    pub proxy_enforced: bool,
    pub network_proxy_restricting_sid: Option<&'a str>,
    pub proxy_settings_mode: WindowsSandboxProxySettingsMode,
    pub filesystem_overrides: Option<&'a WindowsSandboxFilesystemOverrides>,
    pub use_private_desktop: bool,
}

/// Executor-native process launch request shared by local and exec-server execution.
pub struct SpawnRequest<'a> {
    pub command: &'a [String],
    pub cwd: &'a Path,
    pub env: &'a HashMap<String, String>,
    pub arg0: &'a Option<String>,
    pub sandbox: SandboxType,
    pub windows_sandbox: Option<WindowsSandboxSpawnRequest<'a>>,
    pub tty: bool,
    pub stdin_open: bool,
    pub inherited_fds: &'a [i32],
}

/// Spawn a process using the backend selected by the prepared sandbox request.
pub async fn spawn_process(request: SpawnRequest<'_>) -> Result<SpawnedProcess> {

    let (program, args) = request
        .command
        .split_first()
        .context("missing program for process spawn")?;
    if request.tty {
        codex_utils_pty::pty::spawn_process(
            program,
            args,
            request.cwd,
            request.env,
            request.arg0,
            TerminalSize::default(),
            request.inherited_fds,
        )
        .await
    } else if request.stdin_open {
        codex_utils_pty::pipe::spawn_process(
            program,
            args,
            request.cwd,
            request.env,
            request.arg0,
            request.inherited_fds,
        )
        .await
    } else {
        codex_utils_pty::pipe::spawn_process_no_stdin(
            program,
            args,
            request.cwd,
            request.env,
            request.arg0,
            request.inherited_fds,
        )
        .await
    }
}
