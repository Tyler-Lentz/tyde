// adapted from https://github.com/marc2332/tauri-terminal
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::{
    io::{BufRead, BufReader, Write, Read},
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
use tauri::{async_runtime::Mutex as AsyncMutex, State, Window};

pub struct TerminalState {
    pub pty_pair: Arc<AsyncMutex<PtyPair>>,
    pub writer: Arc<AsyncMutex<Box<dyn Write + Send>>>,
}

#[tauri::command]
pub async fn async_write_to_pty(data: &str, state: State<'_, TerminalState>) -> Result<(), ()> {
    write!(state.writer.lock().await, "{}", data).map_err(|_| ())
}

#[tauri::command]
pub async fn async_resize_pty(rows: u16, cols: u16, state: State<'_, TerminalState>) -> Result<(), ()> {
    state
        .pty_pair
        .lock()
        .await
        .master
        .resize(PtySize {
            rows,
            cols,
            ..Default::default()
        })
        .map_err(|_| ())
}

pub struct Terminal {
    pub reader: Arc<Mutex<Option<BufReader<Box<dyn Read + Send>>>>>,
    pub writer: Box<dyn Write + Send>,
    pub pty_pair: PtyPair,
}

impl Terminal {
    // TODO: handle the errors thrown here without panicking
    pub fn new() -> Self {
        let pty_system = native_pty_system();

        let pty_pair = pty_system
            .openpty(PtySize {
                rows: 12,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();

        #[cfg(target_os = "windows")]
        let cmd = CommandBuilder::new("powershell.exe");
        #[cfg(not(target_os = "windows"))]
        let cmd = CommandBuilder::new("bash");

        let mut child = pty_pair.slave.spawn_command(cmd).unwrap();

        thread::spawn(move || {
            child.wait().unwrap();
        });

        let reader = pty_pair.master.try_clone_reader().unwrap();
        let writer = pty_pair.master.take_writer().unwrap();

        let reader = Arc::new(Mutex::new(Some(BufReader::new(reader))));

        Self {
            reader, writer, pty_pair
        }
    }
}

pub fn reader_thread(window: Window, reader: Arc<Mutex<Option<BufReader<Box<dyn Read + Send>>>>>) {
    thread::spawn(move || {
        let reader = reader.lock().unwrap().take();
        if let Some(mut reader) = reader {
            loop {
                sleep(Duration::from_millis(1));
                let data = reader.fill_buf().unwrap().to_vec();
                reader.consume(data.len());
                if data.len() > 0 {
                    window.emit("terminal-data", data).unwrap();
                }
            }
        }
    });
}