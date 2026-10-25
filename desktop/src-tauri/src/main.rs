#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

#[derive(Serialize)]
struct DesktopInfo {
    version: &'static str,
    engine_available: bool,
}

fn engine_path() -> Result<std::path::PathBuf, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    let directory = executable
        .parent()
        .ok_or("application directory unavailable")?;
    let name = if cfg!(windows) {
        "mshogi.exe"
    } else {
        "mshogi"
    };
    Ok(directory.join(name))
}

#[tauri::command]
fn desktop_info() -> Result<DesktopInfo, String> {
    Ok(DesktopInfo {
        version: env!("CARGO_PKG_VERSION"),
        engine_available: engine_path()?.is_file(),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![desktop_info])
        .run(tauri::generate_context!())
        .expect("mShogi desktop host failed");
}
