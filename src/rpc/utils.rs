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

pub fn get_discord_ipc_pipe() -> PathBuf {
    #[cfg(windows)]
    {
        return PathBuf::from(r"\\?\pipe\discord-ipc-0");
    }

    #[cfg(target_os = "linux")]
    {
        if let Ok(dir) = env::var("XDG_RUNTIME_DIR") {
            return PathBuf::from(dir).join("discord-ipc-0");
        }

        return PathBuf::from("/tmp").join("discord-ipc-0");
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(dir) = env::var("TMPDIR") {
            return PathBuf::from(dir).join("discord-ipc-0");
        }

        return PathBuf::from("/tmp").join("discord-ipc-0");
    }

    #[allow(unreachable_code)]
    PathBuf::new()
}
