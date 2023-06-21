#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod file_system;


fn main() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file_system::drive::get_drive_details, file_system::read_directory])
        .run(tauri::generate_context!())?;

    Ok(())
}
