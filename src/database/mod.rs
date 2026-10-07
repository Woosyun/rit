pub mod blob;
pub mod entry;
pub mod tree;
pub mod oid;

use crate::lockfile;
use std::{error::Error, fs, path};

pub trait IntoObject {
    fn into_object(self) -> String;
}

const DATABASE_DIR: &'static str = "objects";
pub struct Database {
    path: path::PathBuf,
}
impl Database {
    pub fn new<P: AsRef<path::Path>>(repo: P) -> Self {
        let path = repo.as_ref().join(DATABASE_DIR);
        Self { path }
    }

    pub fn store(&self, object: impl IntoObject) -> Result<(), Box<dyn Error>> {
        let obj = object.into_object();
        let oid = oid::Oid::new(&obj)?;

        let target_dir = self.path.join(oid.dir_name());
        fs::create_dir(&target_dir)?;
        let target_path = target_dir.join(oid.file_name());
        let mut lf = lockfile::LockFile::new(target_path);
        lf.load_mut()?;
        todo!("check content format and compress with zlib");
        //lf.write(obj.as_bytes())?;
    }
    pub fn retrieve() {}
}
