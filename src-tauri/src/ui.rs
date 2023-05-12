use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};
use tauri::api::dialog::{FileDialogBuilder, MessageDialogBuilder};
use crate::filesystem;

fn build_menu() -> Menu {
    let open = CustomMenuItem::new("open", "Open");
    let save = CustomMenuItem::new("save", "Save");
    let file_menu = Submenu::new("File", Menu::new().add_item(open).add_item(save));
    Menu::new().add_submenu(file_menu)
}

fn handle_menu_events(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "open" => {
            FileDialogBuilder::new()
                .set_title("Select File to Open")
                .pick_file(move |file_path| {
                    let window = event.window();
                    if let Some(file_path) = file_path {
                        match filesystem::open_file(file_path) {
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
        "save" => {
            let res = event.window().emit("save-file", ());
            if let Err(e) = res {
                eprintln!("{}", e);
            }
        },
        _ => {},
    }
}

pub fn show() {
    tauri::Builder::default()
        .menu(build_menu())
        .on_menu_event(handle_menu_events)
        .invoke_handler(tauri::generate_handler![filesystem::save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}