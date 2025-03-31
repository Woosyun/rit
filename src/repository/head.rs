use std::path::PathBuf;
use crate::{
    workspace::lockfile,
    fs,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Head {
    path: PathBuf,
}
impl Head {
    pub fn name() -> &'static str {
        "HEAD"
    }
    pub fn new(repo: PathBuf) -> Self {
        let mut path = repo;
        path.push(Head::name());
        
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
        lockfile::write(&self.path, branch)?;

        Ok(())
    }
}
