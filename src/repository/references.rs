use crate::prelude::*;
use std::{fs, io, path};

pub struct References {
    path: path::PathBuf,
}
impl References {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }

    pub fn read(&self, relative_path: &RelativePath) -> io::Result<Reference> {
        let target_path: path::PathBuf = self.path.join(relative_path);
        let target_raw = fs::read(target_path)?;
        Reference::parse(target_raw)
    }

    pub fn write(&self, relative_path: &RelativePath, oid: ObjectId) -> io::Result<()> {
        let target_path = self.path.join(relative_path);
        fs::write(target_path, oid)
    }
}
