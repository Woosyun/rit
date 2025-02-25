use crate::{
    traits::*,
    hash256::Hash256,
};
use std::{
    path::PathBuf,
    io::{self, Read},
    fs,
};

pub struct Database {
    path: PathBuf
}
impl Database {
    pub fn build(path: PathBuf) -> Self {
        Self { path }
    }
    
    pub fn folder_name() -> String {
        "objects".to_string()
    }

    // todo: what name should temporary file have?
    pub fn temporary_file_name() -> String {
        "tmp".to_string()
    }

    pub fn store<O: Objectify + CalculateHash>(&self, obj: &O) -> io::Result<()> {
        // Create temporary file and rename it.
        // In this way, race condition will not happen?

        // todo: move hashing into class scope.
        let hash = Hash256::build(obj.calculate_hash());
        let (dir, file) = hash.split();

        let mut dir_path = self.path.clone();
        dir_path.push(dir);

        let mut tmp_path = dir_path.clone();
        tmp_path.push(Database::temporary_file_name());
        if fs::exists(&tmp_path)? {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Temporary file already exists."));
        }
        let _ = fs::File::create(&tmp_path)?;
        let _ = fs::write(&tmp_path, &obj.objectify())?;

        let mut target_path = dir_path.clone();
        target_path.push(file);
        let _ = fs::rename(&tmp_path, &target_path)?;

        Ok(())
    }

    pub fn retrieve<O: Objectify + CalculateHash>(&self, obj: &O) -> io::Result<String> {
        let hash = Hash256::build(obj.calculate_hash());
        let (dir, file) = hash.split();

        let mut target_path = self.path.clone();
        target_path.push(dir);
        target_path.push(file);

        let mut content = String::new();
        let mut file =  fs::File::open(&target_path)?;
        let _ = file.read_to_string(&mut content);

        Ok(content)
    }
}
