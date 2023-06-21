use windows::Win32::Storage::FileSystem::DISK_SPACE_INFORMATION;
use serde::Serialize;
use std::{io, ffi::CString};


/// Lookup [`str`] for finding the corresponding alphabet character from an index.
/// 
/// Used in [`get_drive_details`].
pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";


#[derive(Debug, Serialize)]
pub struct DriveDetails {
    drive_index: u32,
    drive: char,
    len: u64,
}


#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum GetDriveDetailsError {
    IoError(IoErrorWrapper),
    ApiError(&'static str),
    UnknownDrive,
}


#[derive(Debug)]
pub struct IoErrorWrapper(io::Error);


impl Serialize for IoErrorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let as_debug = format!("{:?}", self.0);
        serializer.serialize_str(&as_debug)
    }
}


#[tauri::command]
pub fn get_drive_details() -> Result<Vec<DriveDetails>, GetDriveDetailsError> {
    // SAFETY: Error checking in the below if statement.
    let logical_drives = unsafe { windows::Win32::Storage::FileSystem::GetLogicalDrives() };

    if logical_drives == 0 {
        // TODO: Add more description to this error via `Win32::GetLastError()`
        return Err(GetDriveDetailsError::ApiError("Failed to successfully call `Win32::GetLogicalDrives()`"));
    }

    let mut drive_details = Vec::new();

    for i in 0 .. u32::BITS {
        let mask = 1 << i;

        if (mask & logical_drives) > 0 {
            let Some(drive) = ALPHABET.chars().nth(i as usize) else {
                // TODO: How does Windows behave when a drive is created past `Z`?
                return Err(GetDriveDetailsError::UnknownDrive);
            };

            let mut disk_space_information = DISK_SPACE_INFORMATION::default();

            let drive_path = CString::new(drive.to_string() + ":\\").unwrap();
            let raw_drive_path = drive_path.as_bytes();

            // SAFETY: Error checking in the below if statement.
            let space_information_result = unsafe {
                windows::Win32::Storage::FileSystem::GetDiskSpaceInformationA(
                    windows::core::PCSTR::from_raw(raw_drive_path.as_ptr()),
                    &mut disk_space_information as *mut DISK_SPACE_INFORMATION
                )
            };

            if let Err(_) = space_information_result {
                return Err(GetDriveDetailsError::ApiError("Failed to successfully call `Win32::GetDiskSpaceInformationA()`"));
            }

            let details = DriveDetails {
                len: disk_space_information.ActualAvailableAllocationUnits,
                drive_index: i,
                drive,
            };

            drive_details.push(details);
        }
    }

    Ok(drive_details)
}