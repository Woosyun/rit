pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod commit;
pub use commit::*;

pub mod hash256;
pub use hash256::*;

pub mod database;
pub use database::*;

pub mod traits;
pub use traits::*;


use std::{
    path::PathBuf,
    io,
};

const OBJECTS:&str = "objects";
const _HEAD: &str = "HEAD";

pub struct Repository {
    pub path: PathBuf,
}
impl Repository {
    pub fn build(path: PathBuf) -> io::Result<Self> {
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "cannot fild .rit folder"));
        }
        let re = Self {
            path
        };

        Ok(re)
    }

    pub fn check_health(&self) -> io::Result<&'static str> {
        Ok("ok")
    }

    pub fn get_database(&self) -> io::Result<Database> {
        let mut path = self.path.clone();
        path.push(OBJECTS);

        Database::build(path)
    }

}
