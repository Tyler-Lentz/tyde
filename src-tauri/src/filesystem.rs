use std::{fs, path::PathBuf};

#[derive(serde::Serialize, Clone)]
pub struct FileMessage {
    pub file: String,
}

pub fn open_file(fname: PathBuf) -> Result<String, String> {
    Ok(
        fs::read_to_string(fname)
            .map_err(|e| e.to_string())?
    )
}