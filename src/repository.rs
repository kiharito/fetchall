use crate::directories::Directory;
use anyhow::Result;
use std::fs::OpenOptions;

pub trait Repository {
    fn collect(&self) -> Result<Vec<Directory>>;
    fn save(&self, _: &[Directory]) -> Result<()>;
}

pub struct JsonFileRepository<'a> {
    file_path: &'a str,
}

impl JsonFileRepository<'_> {
    pub fn new(file_path: &str) -> JsonFileRepository {
        JsonFileRepository { file_path }
    }
}

impl Repository for JsonFileRepository<'_> {
    fn collect(&self) -> Result<Vec<Directory>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(self.file_path)?;
        match serde_json::from_reader(file) {
            Ok(dirs) => Ok(dirs),
            Err(e) if e.is_eof() => Ok(Vec::new()),
            Err(e) => Err(e)?,
        }
    }
    fn save(&self, dirs: &[Directory]) -> Result<()> {
        let file = OpenOptions::new().write(true).open(self.file_path)?;
        file.set_len(0)?;
        serde_json::to_writer(file, &dirs)?;
        Ok(())
    }
}
