use crate::prelude::*;
use std::io::Write;
use std::{fs, io, path};

pub struct Database {
    path: path::PathBuf,
}
impl Database {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }
    pub fn store(&self, obj: Object) -> io::Result<ObjectId> {
        let obj_raw = <ObjectRaw as From<Object>>::from(obj);
        let oid = hash(&obj_raw.data);
        let oid_str = <String as From<&ObjectId>>::from(&oid);

        let compressed = compress(&obj_raw.data)?;

        let dir = self.path.join(&oid_str[..2]);
        if !dir.exists() {
            fs::create_dir(&dir)?;
        }
        let target = dir.join(&oid_str[2..]);
        //if target is exists, finish. Return what?
        let mut lf = LockFile::new(target);
        lf.hold()?;
        lf.write_all(&compressed)?;
        lf.commit()?;

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
