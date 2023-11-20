use std::{fs::{self}, path::{PathBuf, Path}, sync::{Arc, Mutex}, pin::Pin};
use std::io;
use tauri::{api::dialog::FileDialogBuilder, Window};
use notify::{Event, EventKind, INotifyWatcher};


pub struct FileWatchInfo {
    pub watcher: Option<INotifyWatcher>,
    pub dir: Option<PathBuf>,
}

pub struct FileWatchState {
    pub info: Pin<Arc<Mutex<FileWatchInfo>>>,
}


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
pub fn save_file(fpath: String, content: String) -> Result<String, String> {
    fs::write(fpath.clone(), content).map_err(|e| e.to_string())?;
    Ok(fpath)
}

#[tauri::command]
pub fn save_as_file(content: String, window: tauri::Window) {
    FileDialogBuilder::new()
        .set_title("Save as")
        .save_file(move |file_path| {
            if let Some(file_path) = file_path {
                let res = fs::write(file_path.clone(), content).map_err(|e| e.to_string());
                if let Err(e) = window.emit("save-as-file-completed", res.and(Ok(file_path.to_str()))) {
                    eprintln!("{}", e);
                }
            }
        });
}

pub fn dir_listen(e: Event, window: &Window) {
    match e.kind {
        EventKind::Create(_) |
        EventKind::Remove(_) => {
            window.emit("test", ());
        },
        _ => {}
    }
}