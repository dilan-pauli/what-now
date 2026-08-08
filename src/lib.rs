use std::env;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};

// Make sure there is a file at ~/.config/whatnow
pub fn ensure_file(home_dir: &Path) -> io::Result<PathBuf> {
    let mut path_to_config = home_dir.to_path_buf();
    path_to_config.push(".config");
    path_to_config.push("whatnow");

    if path_to_config.exists() {
        return Ok(path_to_config);
    }

    if let Some(parent) = path_to_config.parent() {
        fs::create_dir_all(parent)?;
    }

    File::create(&path_to_config)?;

    Ok(path_to_config)
}

// Gets the user home path for linux and Windows
pub fn get_muli_os_home() -> Option<PathBuf> {
    env::var_os("HOME")
        .or_else(|| env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}
