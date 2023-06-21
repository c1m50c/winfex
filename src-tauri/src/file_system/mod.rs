use std::os::windows::prelude::MetadataExt;
use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;

pub mod drive;


#[derive(Debug, Serialize)]
pub struct DirectoryRecord {
    name: String,
    path: String,

    #[serde(skip_serializing_if  = "Option::is_none")]
    size: Option<u64>,

    #[serde(skip_serializing_if  = "Option::is_none")]
    is_directory: Option<bool>,
}


#[derive(Debug)]
pub struct SerializableIoError(io::Error);


impl From<io::Error> for SerializableIoError {
    fn from(value: io::Error) -> Self {
        Self(value)
    }
}


impl Serialize for SerializableIoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(&self.0.to_string())
    }
}


#[tauri::command]
pub fn read_directory(path: String) -> Result<Vec<DirectoryRecord>, SerializableIoError> {
    let mut records = Vec::new();
    let path = Path::new(&path);

    if path.is_file() {
        return Ok(records);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;

        let name = entry.file_name().to_str()
            .unwrap_or_default()
            .to_string();

        let path = entry.path().to_str()
            .unwrap_or_default()
            .to_string();

        let size = entry.metadata()
            .map(|ok| ok.file_size())
            .ok();

        let is_directory = entry.file_type()
            .map(|ok| ok.is_dir())
            .ok();

        let record = DirectoryRecord { name, path, size, is_directory };
        records.push(record);
    }

    Ok(records)
}