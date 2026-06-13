use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_last_journal(dir: impl AsRef<Path>) -> io::Result<PathBuf> {
    fs::read_dir(dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "log"))
        .max()
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "log file not found",
        ))
}

pub fn get_discord_ipc_pipe() -> PathBuf {
    #[cfg(target_os = "linux")]
    let base = PathBuf::from(std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into()));

    #[cfg(target_os = "macos")]
    let base = PathBuf::from(std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".into()));

    base.join("discord-ipc-0")
}
