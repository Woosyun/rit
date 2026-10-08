pub mod blob;
pub mod entry;
pub mod oid;
pub mod tree;

use crate::lockfile;
use std::{error::Error, fs, path};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

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
        let content = obj.as_bytes();
        let oid = oid::Oid::new(&obj)?;

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(content)?;
        let compressed = e.finish()?;

        let target_dir = self.path.join(oid.dir_name());
        fs::create_dir(&target_dir)?;
        let target_path = target_dir.join(oid.file_name());
        let mut lf = lockfile::LockFile::new(target_path);
        lf.load_mut()?;
        lf.write(&compressed)?;

        Ok(())
    }
    pub fn retrieve() {}
}
