use std::process::{Stdio};

use execute::{Execute, shell};

// Based on the example here https://docs.rs/execute/latest/execute/
#[tauri::command]
pub fn shell_exec(command: String) -> Result<String, String> {
    let mut command = shell(command);

    command.stdout(Stdio::piped());

    let output = command.execute_output().unwrap();

    return String::from_utf8(output.stdout).map_err(|e| e.to_string());
}