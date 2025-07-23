use std::{
    fs,
    path::PathBuf,
    collections::HashSet,
};
use crate::{
    prelude::*,
    error::*,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch(Oid, Oid);
impl Branch {
    pub fn new(oid: &Oid) -> Self {
        Self(oid.clone(), oid.clone())
    }

    pub fn root(&self) -> &Oid {
        &self.0
    }
    pub fn leaf(&self) -> &Oid {
        &self.1
    }
    pub fn update(&mut self, oid: &Oid) {
        self.1 = oid.clone();
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn build(repo: PathBuf) -> crate::Result<Self> {
        let mut path = repo;
        path.push(Refs::name());
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
        path.push(Refs::name());
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Refs(e.to_string()))?;
        }

        path.push(Refs::local());
        if !path.exists() {
            fs::create_dir(&path)
                .map_err(|e| Error::Refs(e.to_string()))?;
        }
        Ok(())
    }

    pub fn contains(&self, branch: &str) -> bool {
        let mut path = self.path.clone();
        path.push(Refs::local());
        path.push(branch);
        path.exists()
    }

    pub fn list_branches(&self) -> Result<HashSet<String>> {
        let mut path = self.path.clone();
        path.push(Refs::local());

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

    pub fn get(&self, branch: &str) -> crate::Result<Branch> {
        let mut path = self.path.clone();
        path.push(Refs::local());
        path.push(branch);
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Refs(e.to_string()))?;
        let tip = serde_json::from_str(&content)
            .map_err(|e| Error::Refs(e.to_string()))?;

        Ok(tip)
    }

    pub fn set(&self, branch: &str, oid: &Oid) -> crate::Result<()> {
        let mut path = self.path.clone();
        path.push(Refs::local());
        path.push(branch);

        let branch = if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| Error::Refs(e.to_string()))?;
            let mut branch: Branch = serde_json::from_str(&content)
                .map_err(|e| Error::Refs(e.to_string()))?;
            branch.update(oid);

            branch
        } else {
            let new_branch = Branch::new(oid);
            new_branch
        };

        let content = serde_json::to_string(&branch)
            .map_err(|e| Error::Refs(e.to_string()))?;
        lock_write(&path, &content)
            .map_err(|e| Error::Refs(e.to_string()))?;
        Ok(())
    }
    
}
