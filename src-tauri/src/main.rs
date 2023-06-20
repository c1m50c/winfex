#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .run(tauri::generate_context!())?;

    Ok(())
}
