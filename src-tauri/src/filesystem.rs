use std::{fs::{self}, path::{PathBuf, Path}};
use std::io;

#[derive(serde::Serialize, Clone)]
pub struct FileMessage {
    pub content: String,
    pub name: String,
}

#[derive(serde::Serialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum Node {
    Dir(String, Vec<Node>),
    File(String),
}

#[derive(serde::Serialize, Clone)]
pub struct DirMessage {
    pub root: Node,
}

pub fn open_file(fname: PathBuf) -> Result<FileMessage, String> {
    Ok(FileMessage {
        content: fs::read_to_string(fname.clone()).map_err(|e| e.to_string())?,
        name: fname.display().to_string()
    })
}

pub fn open_dir(dir_name: PathBuf) -> Result<DirMessage, String> {
    let root = visit_dirs(dir_name.as_path()).map_err(|e| e.to_string())?;
    Ok(DirMessage{root})
}

fn visit_dirs(dir: &Path) -> io::Result<Node> {
    let name = dir.to_string_lossy().to_string();
    if dir.is_dir() {
        let mut children = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let curr_name = path.to_string_lossy().to_string();
            if path.is_dir() {
                children.push(visit_dirs(&path)?);
            } else {
                children.push(Node::File(curr_name))
            }
        }
        Ok(Node::Dir(name, children))
    } else {
        Ok(Node::File(name))
    }
}


#[tauri::command]
pub fn save_file(fpath: String, content: String) -> Result<(), String> {
    fs::write(fpath, content).map_err(|e| e.to_string())?;
    Ok(())
}