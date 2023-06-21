#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod file_system;


fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_system::get_drive_details])
        .run(tauri::generate_context!())?;

    Ok(())
}
