use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const APP_DATA_DIR_NAME: &str = ".kusanaginokajiki";

pub fn read_state<'a, T>(
    lock: &'a RwLock<T>,
    label: &'static str,
) -> Result<RwLockReadGuard<'a, T>, String> {
    lock.read()
        .map_err(|e| format!("{label} state lock poisoned: {e}"))
}

pub fn write_state<'a, T>(
    lock: &'a RwLock<T>,
    label: &'static str,
) -> Result<RwLockWriteGuard<'a, T>, String> {
    lock.write()
        .map_err(|e| format!("{label} state lock poisoned: {e}"))
}

pub fn mutex_state<'a, T>(
    lock: &'a Mutex<T>,
    label: &'static str,
) -> Result<MutexGuard<'a, T>, String> {
    lock.lock()
        .map_err(|e| format!("{label} state lock poisoned: {e}"))
}

pub fn create_dir_all(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| format!("create_dir_all {}: {e}", path.display()))
}

pub fn app_data_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "Could not determine home directory".to_string())?;
    Ok(home.join(APP_DATA_DIR_NAME))
}

pub fn read_text_file(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))
}

pub fn write_text_file(path: &Path, content: &str) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| format!("write {}: {e}", path.display()))
}

pub fn write_bytes_file(path: &Path, content: &[u8]) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| format!("write {}: {e}", path.display()))
}
