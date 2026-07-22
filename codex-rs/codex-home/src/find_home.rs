use codex_utils_absolute_path::AbsolutePathBuf;
use std::path::PathBuf;

pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let codex_home_env = std::env::var("CODEX_HOME")
        .ok()
        .filter(|val| !val.is_empty());
    find_codex_home_from_env(codex_home_env.as_deref())
}

fn find_codex_home_from_env(codex_home_env: Option<&str>) -> std::io::Result<AbsolutePathBuf> {
    let raw_path = match codex_home_env {
        Some(val) => PathBuf::from(val),
        None => {
            // Android App internal storage path: if TMPDIR is set to /data/data/<pkg>/cache, derive /data/data/<pkg>/files
            if let Ok(tmp) = std::env::var("TMPDIR") {
                let tmp_path = PathBuf::from(tmp);
                if let Some(parent) = tmp_path.parent() {
                    parent.join("files")
                } else {
                    tmp_path.join("files")
                }
            } else {
                PathBuf::from("/data/data/com.codex.agent/files")
            }
        }
    };

    let _ = std::fs::create_dir_all(&raw_path);
    let canonical = raw_path.canonicalize().unwrap_or(raw_path);
    AbsolutePathBuf::from_absolute_path(canonical)
}
