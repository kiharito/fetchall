use std::fs::OpenOptions;
use std::io::{Result, Write};

pub fn add_directory_path(file_path: &str, path: String) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;
    let path_str = path + "\n";
    file.write_all(path_str.as_ref())?;
    Ok(())
}
