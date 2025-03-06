pub mod database;
pub use database::*;

use std::{
    path::PathBuf,
    io,
    fs,
};

const OBJECTS:&str = "objects";
const _HEAD: &str = "HEAD";
const RIT: &str = ".rit";

pub struct Repository {
    pub path: PathBuf,
}
impl Repository {
    pub fn find(workdir: PathBuf) -> io::Result<Self> {
        let mut path = workdir;
        path.push(RIT);
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "cannot find .rit"));
        }
        let re = Self {
            path
        };
        Ok(re)
    }

    pub fn init(workdir: PathBuf) -> io::Result<&'static str> {
        let mut path = workdir;
        path.push(RIT);
        if path.exists() {
            return Ok(".rit already exists");
        }

        let mut obj = path.clone();
        obj.push(OBJECTS);
        let _ = fs::create_dir(obj)?;

        Ok("initialized")
    }

    /*
    pub fn get_database(&self) -> io::Result<Database> {
        let mut path = self.path.clone();
        path.push(OBJECTS);
        if path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "cannot find objects folder in repository"));
        }

        Ok(Database::new(path))
    }
    */

}
