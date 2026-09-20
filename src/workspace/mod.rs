use std::{fs, path, io};

pub struct Workspace {
    working_directory: path::PathBuf
}
impl Workspace {
    pub fn new<P: AsRef<path::Path>>(working_directory: P) -> Result<Self, io::Error /*변경 가능*/ > {
        let working_directory = working_directory.as_ref().canonicalize()?;
        let ws = Self {
            working_directory,
        };
        Ok(ws)
    }

    pub fn list_files(&self) -> Result<Vec<path::PathBuf>, io::Error> {
        fs::read_dir(&self.working_directory)?
            .map(|dir_entry_result| dir_entry_result.map(|dir_entry| dir_entry.path()))
            .collect::<Result<Vec<path::PathBuf>, io::Error>>()
    }
}
