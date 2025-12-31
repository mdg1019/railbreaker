use std::path::PathBuf;
use std::sync::OnceLock;
use serde::{de::DeserializeOwned, Serialize};
use tokio::fs;

static WRITE_MUTEX: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

fn write_mutex() -> &'static tokio::sync::Mutex<()> {
    WRITE_MUTEX.get_or_init(|| tokio::sync::Mutex::new(()))
}

pub async fn read_json_file<T>(path: impl Into<PathBuf>) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let path = path.into();
    
    let content = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;
    
    serde_json::from_str::<T>(&content)
        .map_err(|e| format!("Failed to parse JSON file '{}': {}", path.display(), e))
}

pub async fn write_json_file<T>(path: impl Into<PathBuf>, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    let path = path.into();
    let content = serde_json::to_string_pretty(value)
        .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;

    let _guard = write_mutex().lock().await;
    let mut tmp = path.clone();
    tmp.set_extension("tmp");

    fs::write(&tmp, &content)
        .await
        .map_err(|e| format!("Failed to write temp file '{}': {}", tmp.display(), e))?;

    match fs::rename(&tmp, &path).await {
        Ok(()) => Ok(()),
        Err(e) => {
            fs::write(&path, &content)
                .await
                .map_err(|e2| format!("Failed to rename temp file '{}' -> '{}' (rename error: {}), and failed to write directly: {}", tmp.display(), path.display(), e, e2))
        }
    }
}