use crate::prelude::*;
use std::{
    path::PathBuf,
    io::{self, Write},
    fs::{self, File},
};

const RIT: &str = ".rit";
const OBJECTS: &str = "objects";
const HEAD: &str = "HEAD";

pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn build(workdir: PathBuf) -> io::Result<Self> {
        let mut repo = workdir;
        repo.push(RIT);

        if !repo.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "cannot found .rit"));
        }

        let repo = Self {
            path: repo
        };

        Ok(repo)
    }

    pub fn init(workdir: PathBuf) -> io::Result<&'static str> {
        let mut repo = workdir;
        repo.push(RIT);

        if !repo.exists() {
            let _ = fs::create_dir(&repo)?;
        }

        // does file need to be created to be written?

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
        path.push(OBJECTS);
        path.push(&dir);
        path.push(&file);
        if path.exists() {
            return Ok(oid);
        }

        let mut tmp = self.path.clone();
        tmp.push(OBJECTS);
        tmp.push(&dir);
        let _ = fs::create_dir(&tmp)?;
        tmp.push("tmp");
        let mut buffer = File::create(&tmp)?;
        let _ = buffer.write_all(content.as_bytes())?;
        let _ = fs::rename(tmp, path)?;
        Ok(oid)
    }

    pub fn set_head(&self, oid: &Oid) -> io::Result<()> {
        let mut head = self.path.clone();
        head.push(HEAD);

        let mut tmp = self.path.clone();
        tmp.push("tmp");
        let _ = fs::write(&tmp, oid.decode())?;
        let _ = fs::rename(&tmp, head)?;

        Ok(())
    }
    pub fn get_head(&self) -> io::Result<Option<Oid>> {
        let mut head = self.path.clone();
        head.push(HEAD);

        if !head.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&head)?;
        let result = if content.is_empty() {
            None
        } else {
            let oid = Oid::encode(&content)?;
            Some(oid)
        };

        Ok(result)
    }
}
