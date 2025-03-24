pub mod tree;
pub use tree::*;

pub mod blob;
pub use blob::*;

pub mod entry;
pub use entry::*;

pub mod oid;
pub use oid::*;

use std::{
    io,
    fs,
    path::PathBuf,
};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::workspace::lockfile;

pub const OBJECTS: &str = "objects";

pub struct Database {
    path: PathBuf
}
impl Database {
    pub fn build(repo: PathBuf) -> io::Result<Self> {
        let mut path = repo;
        path.push(OBJECTS);

        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, ".rit/objects not found"));
        }

        let db = Self {
            path
        };
        Ok(db)
    }
    pub fn store(&self, oid: &Oid, content: String) -> io::Result<()> {
        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(OBJECTS);
        path.push(dir);
        fs::create_dir(&path)?;

        path.push(file);

        if path.exists() {
            return Ok(());
        }
        
        lockfile::store(&path, content)?;
        Ok(())
    }
}

pub trait Objectify<'a>: Serialize + Deserialize<'a>{
    fn get_oid(content: &str) -> io::Result<oid::Oid> {
        let oid = oid::Oid::build(&content);
        Ok(oid)
    }

    fn decode(&self) -> io::Result<String> {
         serde_json::to_string(self)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "serialization failed"))
    }

    fn encode<O: Deserialize<'a>>(content: &'a str) -> io::Result<O> {
        serde_json::from_str(content)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "deserialization failed"))
    }
}
