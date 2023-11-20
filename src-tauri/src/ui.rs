use notify::{Watcher, RecursiveMode};
use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent, Manager, State};
use tauri::api::dialog::{FileDialogBuilder, MessageDialogBuilder};
use crate::filesystem::{self, FileWatchState};
use std::sync::{Arc, Mutex};
use std::pin::Pin;


pub fn build_menu() -> Menu {
    let open = CustomMenuItem::new("open", "Open File...");
    let save = CustomMenuItem::new("save", "Save");
    let save_as = CustomMenuItem::new("save-as", "Save As...");
    let new = CustomMenuItem::new("new", "New File...");
    let directory = CustomMenuItem::new("open-directory", "Open Directory...");
    let file_menu = Submenu::new("File", 
        Menu::new()
            .add_item(new)
            .add_item(open)
            .add_item(directory)
            .add_item(save)
            .add_item(save_as)
    );
    Menu::new().add_submenu(file_menu)
}

pub fn handle_menu_events(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open" => {
            FileDialogBuilder::new()
                .set_title("Select File to Open")
                .pick_file(move |file_path| {
                    let window = event.window();
                    if let Some(file_path) = file_path {
                        match filesystem::open_file(file_path.to_string_lossy().to_string()) {
                            Ok(message) => {
                                let res = window.emit("open-file", message);
                                if let Err(e) = res {
                                    eprintln!("{}", e);
                                }
                            },
                            Err(e) => {
                                MessageDialogBuilder::new("Error", &e.to_string())
                                    .show(|_| {});
                            }
                        }
                    }
                });
        },
        "new" => {
            let res = event.window().emit("new-file", ());
            if let Err(e) = res {
                eprintln!("{}", e);
            } 
        },
        "save" => {
            let res = event.window().emit("save-file", ());
            if let Err(e) = res {
                eprintln!("{}", e);
            }
        },
        "save-as" => {
            let res = event.window().emit("save-as-file", ());
            if let Err(e) = res {
                eprintln!("{}", e);
            }
        },
        "open-directory" => {
            FileDialogBuilder::new()
                .set_title("Select Directory to Open")
                .pick_folder(move |dir_path| {
                    let window = event.window().clone();
                    if let Some(dir_path) = dir_path {
                        match filesystem::open_dir(dir_path.clone()) {
                            Ok(message) => {
                                let res = window.emit("open-directory", message);

                                let handle = window.app_handle();
                                let state: State<'_, FileWatchState> = handle.state();
                                let mut state = state.info.lock().unwrap();

                                match &mut state.watcher {
                                    None => {
                                        // Set up listener function since this is the first time
                                        state.watcher = Some(notify::recommended_watcher(move |e| {
                                            if let Ok(e) = e {
                                                filesystem::dir_listen(e, &window);
                                            }
                                        }).unwrap());
                                    },
                                    Some(watcher) => {
                                        // Unwatch old path
                                    }
                                }

                                state.dir = Some(dir_path.clone());

                                if let Some(watcher) = &mut state.watcher {
                                    watcher.watch(dir_path.as_path(), RecursiveMode::Recursive);
                                }

                                if let Err(e) = res {
                                    eprintln!("{}", e);
                                }
                            },
                            Err(e) => {
                                MessageDialogBuilder::new("Error", &e.to_string())
                                    .show(|_| {});
                            }
                        }
                    }
                });
        }
        _ => {},
    }
}