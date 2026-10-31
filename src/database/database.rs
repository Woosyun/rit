use crate::{lockfile, zlib};
use std::{error, fs, path};

#[allow(unused)]
use std::io::{Read, Write};

/// Trait for transforming
/// from Blob/Tree/Commit
/// to raw object to be stored.
pub trait IntoObject {
    fn into_object(self) -> Vec<u8>;
}

/// Trait for transforming
/// from raw object
/// to Blob/Tree/Commit.
pub trait FromObject {
    fn from_object<U: AsRef<[u8]>>(content: U) -> Self;
}

/// Default folder name for database.
/// Same as "objects" in git.
pub const DIR_NAME: &'static str = "objects";

/// Responsible for dealing with database.
pub struct Database {
    path: path::PathBuf,
}
impl Database {
    pub fn new<P: AsRef<path::Path>>(repodir: P) -> Self {
        let path = repodir.as_ref().join(DIR_NAME);
        Self { path }
    }
    //todo: split function for testing
    pub fn store(&self, object: impl IntoObject) -> Result<super::Oid, Box<dyn error::Error>> {
        let obj = object.into_object();
        let oid = super::Oid::new(&obj)?;

        let compressed = zlib::compress(&obj)?;

        let target_dir = self.path.join(oid.dir_name());
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
        }
        let target_path = target_dir.join(oid.file_name());
        //if target is exists, finish
        let mut lf = lockfile::LockFile::new(target_path);
        lf.hold()?;
        lf.write(&compressed)?;

        Ok(oid)
    }

    /*
    pub fn retrieve(&self, oid: oid::Oid) -> Result<impl IntoObject, Box<dyn error::Error>> {
        let target_path = self.path.join(oid.dir_name()).join(oid.file_name());
        let compressed = fs::read(target_path)?;

        let mut decompressed: Vec<u8> = Vec::new();
        let mut decoder = ZlibDecoder::new(&decompressed[..]);
        decoder.read_to_end(&mut compressed)?;

        let object_type = decompressed

        //todo: read type, size, content

        Ok()
    }
    */
}
