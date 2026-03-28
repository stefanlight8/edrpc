use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[inline]
pub fn get_nonce() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .to_string()
}

pub fn get_discord_ipc_pipe() -> Option<PathBuf> {
    #[cfg(windows)]
    let base = PathBuf::from(r"\\?\pipe");

    #[cfg(target_os = "linux")]
    let base = PathBuf::from(env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into()));

    #[cfg(target_os = "macos")]
    let base = PathBuf::from(env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into()));

    let mut last = None;

    for i in 0..10 {
        let candidate = base.join(format!("discord-ipc-{}", i));
        if candidate.exists() {
            last = Some(candidate);
        }
    }

    last
}
