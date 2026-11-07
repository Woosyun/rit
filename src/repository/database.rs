use crate::prelude::*;
use std::io::Write;
use std::{error, fs, io, path};

pub struct Database {
    path: path::PathBuf,
}
impl Database {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }
    pub fn store(&self, obj: Object) -> io::Result<ObjectId> {
        //let obj: Object = object.into();
        let oid: [u8; 20] = hash(obj.as_slice());

        let compressed = compress(&obj)?;

        let dir_name: String = oid
            .iter()
            .take(2)
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let dir = self.path.join(dir_name);
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        let file_name = oid
            .iter()
            .skip(2)
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        let target = dir.join(file_name);
        //if target is exists, finish. Return what?
        let mut lf = LockFile::new(target);
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
