use crate::states::global_state::global_state;
use tokio::fs;

#[tauri::command]
pub async fn process_zip_file(path: String) -> Result<String, String> {
    let file = std::fs::File::open(&path).map_err(|e| format!("Failed to open zip file: {}", e))?;

    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read zip archive: {}", e))?;

    if archive.len() == 0 || archive.len() > 1 {
        return Err("Zip archive is not a valid single file archive".to_string());
    }

    let mut file = archive
        .by_index(0)
        .map_err(|e| format!("Failed to access file in zip: {}", e))?;

    let enclosed_name = file.enclosed_name();
    let filename = match enclosed_name.as_ref() {
        Some(path) => path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?,
        None => return Err("File has no valid enclosed name".to_string()),
    };

    if !filename.to_lowercase().ends_with(".drf") {
        return Err(format!(
            "File must have a .DRF extension, got: {}",
            filename
        ));
    }

    let racecards_dir = {
        let global_state = global_state()
            .lock()
            .map_err(|e| format!("Failed to lock global state: {}", e))?;
        format!("{}/Racecards", global_state.current_directory)
    };

    let outpath = std::path::PathBuf::from(&racecards_dir).join(filename);

    if (*file.name()).ends_with('/') {
        return Err(format!("Zip contains directories, which is not supported"));
    } else {
        use std::io::Read;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file from zip: {}", e))?;

        fs::write(&outpath, buffer)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;
    }

    Ok(outpath.to_string_lossy().to_string())
}
