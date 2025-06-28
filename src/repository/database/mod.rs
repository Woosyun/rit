pub mod tree;

pub mod blob;
pub use blob::*;

pub mod entry;
pub use entry::*;

pub mod oid;
pub use oid::*;

pub mod commit;
pub use commit::*;


use std::{
    fs,
    path::PathBuf,
};
use crate::prelude::*;
use serde::{Serialize, de::DeserializeOwned, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    path: PathBuf
}
impl Database {
    pub fn name() -> &'static str {
        "objects"
    }

    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(Database::name());
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
        path.push(Database::name());
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Database(e.to_string()))?;
        }
        Ok(())
    }

    pub fn store<O: Serialize>(&self, o: &O) -> crate::Result<Oid> {
        let content = serde_json::to_string(o)
            .map_err(|e| Error::Database(e.to_string()))?;
        let oid = Oid::build(&content);

        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(dir);
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Database(e.to_string()))?;
        }
        path.push(file);
        if path.exists() {
            return Ok(oid);
        }

        lock_write(&path, &content)
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(oid)
    }

    pub fn retrieve<O: DeserializeOwned>(&self, oid: &Oid) -> Result<O> {
        let mut path = self.path.clone();
        let (dir, file) = oid.split();
        path.push(dir);
        path.push(file);

        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Database(e.to_string()))?;
        let obj: O = serde_json::from_str(&content)
            .map_err(|e| Error::Database(e.to_string()))?;
        Ok(obj)
    }
}
