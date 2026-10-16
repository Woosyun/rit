use crate::database;
use std::{io, path};

pub const DIR_NAME: &'static str = "refs";
pub const HEAD_FILE_NAME: &'static str = "HEAD";

pub struct Refs {
    repo_dir: path::PathBuf,
}
impl Refs {
    pub fn new<P: AsRef<path::Path>>(repo_dir: P) -> Self {
        Self {
            repo_dir: repo_dir.as_ref().to_path_buf(),
        }
    }
    pub fn read_head(&self) -> io::Result<Ref> {
        let tmp = Ref::Oid(Default::default());
        Ok(tmp)
    }
    pub fn update_head(&self, oid: database::Oid) -> io::Result<()> {
        todo!()
    }
}

pub enum Ref {
    Oid(database::Oid),
    SymRef(String),
}
impl Ref {
    pub fn oid(&self) -> io::Result<&database::Oid> {
        let oid = match self {
            Self::Oid(oid) => oid,
            _ => panic!("available path yet"),
        };
        Ok(oid)
    }
}
