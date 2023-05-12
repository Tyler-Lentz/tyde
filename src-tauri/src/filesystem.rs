use std::{fs, path::PathBuf};

#[derive(serde::Serialize, Clone)]
pub struct FileMessage {
    pub content: String,
    pub name: String,
}

pub fn open_file(fname: PathBuf) -> Result<FileMessage, String> {
    Ok(FileMessage {
        content: fs::read_to_string(fname.clone()).map_err(|e| e.to_string())?,
        name: fname.display().to_string()
    })
}

#[tauri::command]
pub fn save_file(fpath: String, content: String) -> Result<(), String> {
    fs::write(fpath, content).map_err(|e| e.to_string())?;
    Ok(())
}