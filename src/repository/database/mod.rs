pub mod tree;
pub use tree::*;

pub mod blob;
pub use blob::*;

pub mod entry;
pub use entry::*;

pub mod oid;
pub use oid::*;

pub mod commit;
pub use commit::*;

use std::path::PathBuf;
use crate::{
    workspace::lockfile,
    utils,
    fs,
};
use serde::Serialize;

pub const OBJECTS: &str = "objects";

pub struct Database {
    path: PathBuf
}
impl Database {
    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(OBJECTS);

        if !path.exists() {
            return Err(crate::Error::Repository(".rit/objects not found".into()));
        }

        let db = Self {
            path
        };
        Ok(db)
    }
    pub fn init(repo: PathBuf) -> crate::Result<()> {
        let mut path = repo;
        path.push(OBJECTS);
        fs::create_dir(&path)
    }

    pub fn store<O: Serialize>(&self, o: &O) -> crate::Result<Oid> {
        let content = utils::decode(o)
            .map_err(|e| {
                let msg = format!("cannot decode this object: {}", e);
                crate::Error::Repository(msg)
            })?;
        let oid = Oid::build(&content);

        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(dir);
        if !path.exists() {
            fs::create_dir(&path)?;
        }

        let mut tmp = path.clone();
        tmp.push(&file);
        if tmp.exists() {
            return Ok(oid);
        }

        lockfile::write(&path, &file, &content)?;
        Ok(oid)
    }
}
