use sysinfo::{System, SystemExt, DiskExt};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct DriveDetails {
    available_space: u64,
    total_space: u64,
    path: String,
}


#[tauri::command]
pub fn get_drive_details() -> Vec<DriveDetails> {
    let mut system = System::new_all();
    system.refresh_all();

    let drive_details = system.disks().into_iter()
        .map(|disk| {
            DriveDetails {
                available_space: disk.available_space(),
                total_space: disk.total_space(),
                path: disk.mount_point().to_str()
                    .unwrap_or_default()
                    .to_string(),
            }
        });

    drive_details.collect()
}