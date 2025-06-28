use std::{
    fs,
    path::PathBuf,
};
use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Head {
    Oid(Oid),
    Branch(String),
}
impl Head {
    pub fn is_branch(&self) -> bool {
        match self {
            Head::Branch(_) => true,
            _ => false,
        }
    }
    pub fn branch(&self) -> Result<&str> {
        match self {
            Head::Branch(branch) => Ok(branch),
            _ => Err(Error::Repository("Head is not branch".into())),
        }
    }
    pub fn oid(&self) -> Result<&Oid> {
        match self {
            Head::Oid(oid) => Ok(oid),
            _ => Err(Error::Repository("Head is not oid".into())),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct LocalHead {
    path: PathBuf,
}
impl LocalHead {
    pub fn build(repo: PathBuf) -> Result<Self> {
        let mut path = repo;
        path.push(LocalHead::name());
        if !path.exists() {
            return Err(Error::Repository("LOCAL_HEAD not found".into()));
        }

        Ok(Self {
            path
        })
    }
    pub fn name() -> &'static str {
        "LOCAL_HEAD"
    }
    pub fn init(repo: PathBuf) -> Result<()> {
        let mut path = repo;
        path.push(LocalHead::name());

        if !path.exists() {
            let lh = Self {
                path
            };
            lh.set_to_branch("main")?;
        }

        Ok(())
    }
    pub fn get(&self) -> crate::Result<Head> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| Error::LocalHead(e.to_string()))?;
        let head = serde_json::from_str(&content)
            .map_err(|e| Error::LocalHead(e.to_string()))?;
        Ok(head)
    }
    fn set(&self, head: Head) -> crate::Result<()> {
        let content = serde_json::to_string(&head)
            .map_err(|e| Error::LocalHead(e.to_string()))?;
        lock_write(&self.path, &content)
            .map_err(|e| Error::LocalHead(e.to_string()))?;
        Ok(())
    }

    pub fn set_to_branch(&self, branch: &str) -> Result<()> {
        let head = Head::Branch(branch.to_string());

        self.set(head)
    }
    pub fn set_to_oid(&self, oid: &Oid) -> Result<()> {
        let head = Head::Oid(oid.to_owned());

        self.set(head)
    }
}
