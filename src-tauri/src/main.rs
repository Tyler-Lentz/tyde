// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc,Mutex};
use std::pin::Pin;
use tauri::async_runtime::Mutex as AsyncMutex;

mod filesystem;
mod ui;
mod terminal;
fn main() {
    let terminal = terminal::Terminal::new();

    tauri::Builder::default()
        .menu(ui::build_menu())
        .on_menu_event(ui::handle_menu_events)
        .invoke_handler(tauri::generate_handler![
            filesystem::save_file, filesystem::save_as_file, filesystem::open_file,
            terminal::async_write_to_pty, terminal::async_resize_pty,
        ])
        .manage(terminal::TerminalState {
            pty_pair: Arc::new(AsyncMutex::new(terminal.pty_pair)),
            writer: Arc::new(AsyncMutex::new(terminal.writer)),
        })
        .manage(filesystem::FileWatchState {
            info: Pin::new(Arc::new(Mutex::new(filesystem::FileWatchInfo{watcher: None, dir: None})))
        })
        .on_page_load(move |window, _| {
            terminal::reader_thread(window.clone(), terminal.reader.clone());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
