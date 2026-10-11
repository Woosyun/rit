pub mod blob;
pub use blob::*;
pub mod oid;
pub use oid::*;
pub mod tree;
pub use tree::*;
pub mod commit;
pub use commit::*;

use crate::lockfile;
use std::{error, fs, path};

use flate2::Compression;
#[allow(unused)]
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
#[allow(unused)]
use std::io::{Read, Write};

pub trait IntoObject {
    fn into_object(self) -> Vec<u8>;
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

    //todo: split function for testing
    pub fn store(&self, object: impl IntoObject) -> Result<oid::Oid, Box<dyn error::Error>> {
        let obj = object.into_object();
        let oid = oid::Oid::new(&obj)?;

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
        e.write_all(&obj)?;
        let compressed = e.finish()?;

        let target_dir = self.path.join(oid.dir_name());
        fs::create_dir(&target_dir)?;
        let target_path = target_dir.join(oid.file_name());
        let mut lf = lockfile::LockFile::new(target_path);
        lf.load_mut()?;
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
