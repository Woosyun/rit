pub mod head;
pub use head::*;

pub mod refs;
pub use refs::*;

pub mod database;
pub use database::*;


use std::{
    fs,
    io,
    path::PathBuf,
};

const RIT: &str = ".rit";

pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn build(workdir: PathBuf) -> io::Result<Self> {
        let mut path = workdir;
        path.push(RIT);

        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, ".rit not found"));
        }

        let repo = Self {
            path
        };

        Ok(repo)
    }

    pub fn init(workdir: PathBuf) -> io::Result<&'static str> {
        let mut repo = workdir;
        repo.push(RIT);

        if !repo.exists() {
            let _ = fs::create_dir(&repo)?;
        }

        // does file need to be created to be written? => no


        let mut objects = repo;
        objects.push(OBJECTS);
        let _ = fs::create_dir(objects)?;
        
        Ok("repository is created")
    }
}
