pub mod entry;
pub mod blob;
pub mod tree;

use std::{path, fs, error::Error};
use crate::lockfile;
use sha1::{Sha1, Digest};

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

    pub fn store(&self, object: impl IntoObject) ->Result<(), Box<dyn Error>> {
        let obj = object.into_object();
        let oid = Sha1::digest(&obj)
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let target_dir = self.path.join(&oid[..2]);
        fs::create_dir(&target_dir)?;
        let target_path = target_dir.join(&oid[2..]);
        let mut lf = lockfile::LockFile::new(target_path);
        lf.load_mut()?;
        todo!("check content format and compress with zlib");
        lf.write(obj.as_bytes())?;
    }
    pub fn retrieve() {}
}
