use crate::prelude::*;
use std::{
    path::PathBuf,
    io::{self, Write},
    fs::{self, File},
};

const RIT: &str = ".rit";
const OBJECTS: &str = "objects";

pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn build(workdir: PathBuf) -> io::Result<Self> {
        let mut repo = workdir;
        repo.push(RIT);

        let repo = Self {
            path: repo
        };

        Ok(repo)
    }

    pub fn init(workdir: PathBuf) -> io::Result<&'static str> {
        let mut repo = workdir;
        repo.push(RIT);

        if repo.exists() {
            return Ok("repository already exists");
        }

        let _ = fs::create_dir(&repo)?;

        let mut objects = repo;
        objects.push(OBJECTS);
        let _ = fs::create_dir(objects)?;
        
        Ok("repository is created")
    }

    // oid can be calculated only in here.
    // But storing blob and tree both need oid.
    // So return oid to commander to make entry
    // How about return entry? 
    //   => No, since creating entry needs path value
   pub fn store<O: Objectify>(&self, obj: &O) -> io::Result<Oid> {
        let (oid, content) = obj.to_object();
        let (dir, file) = oid.split();
        let mut path = self.path.clone();
        path.push(&dir);
        path.push(&file);
        if path.exists() {
            return Ok(oid);
        }

        let mut tmp = self.path.clone();
        tmp.push(&dir);
        tmp.push("tmp");
        let mut buffer = File::create(&tmp)?;
        let _ = buffer.write_all(content.as_bytes())?;
        let _ = fs::rename(tmp, path)?;
        Ok(oid)
    }
}
