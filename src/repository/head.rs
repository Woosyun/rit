use std::path::PathBuf;
use crate::fs;
use serde::{Serialize, Deserialize};

const HEAD: &str = "HEAD";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Head {
    path: PathBuf,
}
impl Head {
    pub fn build(refs: PathBuf) -> Self {
        let mut path = refs;
        path.push(HEAD);
        Self {
            path
        }
    }
    pub fn get(&self) -> crate::Result<Option<String>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let branch = fs::read_to_string(&self.path)?;
        Ok(Some(branch))
    }
    pub fn set(&self, branch: &str) -> crate::Result<()> {
        fs::lock_write(&self.path, branch)?;

        Ok(())
    }
}
