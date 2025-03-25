use std::path::PathBuf;
use crate::{
    repository::refs,
    workspace::lockfile,
    fs,
};

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

    pub fn read(&self) -> crate::Result<Option<String>> {
        if !self.path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.path)?;

        Ok(Some(content))
    }
    pub fn write(&self, branch: &str) -> crate::Result<()> {
        let content = refs::REFS.to_owned() + refs::HEADS;
        lockfile::write(&self.path, branch, &content)?;

        Ok(())
    }
}
