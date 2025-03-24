use std::{
    path::PathBuf,
    fs,
    io,
};
use crate::repository::refs;

const HEAD: &str = "HEAD";

pub struct Head {
    path: PathBuf,
}
impl Head {
    pub fn new(repo: PathBuf) -> Self {
        let mut path = repo;
        path.push(HEAD);
        
        Self {
            path
        }
    }

    pub fn get(&self) -> io::Result<Option<String>> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path)?;

        Ok(Some(content))
    }
    // todo: use lock file
    pub fn set(&self, branch: &str) -> io::Result<()> {
        let content = refs::REFS.to_owned() + refs::HEADS + branch;
        let _ = fs::write(&self.path, content)?;

        Ok(())
    }
}
