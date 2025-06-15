use std::{
    fs,
    path::PathBuf,
    collections::HashSet,
};
use crate::prelude::*;
use serde::{Serialize, Deserialize};

const REFS: &str = "refs";
const LOCAL: &str = "local";

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(REFS);
        if !path.exists() {
            return Err(crate::Error::Repository("refs not found".into()));
        }

        Ok(Self {
            path 
        })
    }
    pub fn name() -> &'static str {
        "refs"
    }
    pub fn local() -> &'static str {
        "local"
    }
    pub fn init(repo: PathBuf) -> crate::Result<()> {
        let mut path = repo;
        path.push(REFS);
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Refs(e.to_string()))?;
        }

        path.push(LOCAL);
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Refs(e.to_string()))?;
        }
        Ok(())
    }

    pub fn contains(&self, branch: &str) -> bool {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);
        path.exists()
    }

    pub fn list_branches(&self) -> Result<HashSet<String>> {
        let mut path = self.path.clone();
        path.push(LOCAL);

        let mut result = HashSet::new();
        let read_dir = fs::read_dir(&path)
            .map_err(|e| Error::Refs(e.to_string()))?;
        for entry in read_dir {
            let entry = entry
                .map_err(|e| Error::Refs(e.to_string()))?;
            let branch = entry.file_name()
                .to_str().unwrap()
                .to_string();
            result.insert(branch);
        }

        Ok(result)
    }

    pub fn get(&self, branch: &str) -> crate::Result<Oid> {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Refs(e.to_string()))?;
        let tip = serde_json::from_str(&content)
            .map_err(|e| Error::Refs(e.to_string()))?;

        Ok(tip)
    }

    pub fn set(&self, branch: &str, oid: &Oid) -> crate::Result<()> {
        let mut path = self.path.clone();
        path.push(LOCAL);
        path.push(branch);

        let content = serde_json::to_string(oid)
            .map_err(|e| Error::Refs(e.to_string()))?;
        utils::lock_write(&path, &content)
            .map_err(|e| Error::Refs(e.to_string()))?;
        Ok(())
    }
}
