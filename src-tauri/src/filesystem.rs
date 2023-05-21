use std::{fs::{self}, path::{PathBuf, Path}};
use std::io;

#[derive(serde::Serialize, Clone, Ord, PartialOrd, PartialEq, Eq)]
pub enum FNode {
    Dir{
        path: String, 
        contents: Vec<FNode>
    },
    File{
        path: String,
        content: Option<String>,
    }
}

impl FNode {
    fn from_dir(path: String, contents: Vec<FNode>) -> Self {
        Self::Dir {
            path, contents
        }
    }

    fn from_unloaded_file(path: String) -> Self {
        Self::File {
            path,
            content: None,
        }
    }

    fn from_loaded_file(path: String, content: String) -> Self {
        Self::File {
            path, 
            content: Some(content)
        }
    }
}

#[tauri::command]
pub fn open_file(path: String) -> Result<FNode, String> {
    Ok(
        FNode::from_loaded_file(
            path.clone(),
            fs::read_to_string(path).map_err(|e| e.to_string())?
        )
    )
}

pub fn open_dir(path: PathBuf) -> Result<FNode, String> {
    let root = visit_dirs(path.as_path()).map_err(|e| e.to_string())?;
    Ok(root)
}

fn visit_dirs(dir: &Path) -> io::Result<FNode> {
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
                children.push(FNode::from_unloaded_file(curr_name));
            }
        }
        Ok(FNode::from_dir(name, children))
    } else {
        Ok(FNode::from_unloaded_file(name))
    }
}


#[tauri::command]
pub fn save_file(fpath: String, content: String) -> Result<(), String> {
    fs::write(fpath, content).map_err(|e| e.to_string())?;
    Ok(())
}