pub mod head;
pub use head::*;

pub mod refs;
pub use refs::*;

pub mod database;
pub use database::*;

use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    fs,
};
use crate::prelude::*;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub db: Database,
    pub local_head: LocalHead,
    pub refs: Refs,
}
impl Repository {
    pub fn name() -> &'static str {
        ".rit"
    }
    pub fn build(ws: &Workspace) -> crate::Result<Self> {
        let mut path = ws.workdir().to_path_buf();
        path.push(Repository::name());
        if !path.exists() {
            return Err(crate::Error::Repository("NOT FOUND".into()));
        }

        let db = Database::build(path.clone())?;
        let local_head = LocalHead::build(path.clone())?;
        let refs = Refs::build(path.clone())?;

        let repo = Self {
            db,
            local_head,
            refs,
        };

        Ok(repo)
    }
    pub fn init(ws: &Workspace) -> crate::Result<()> {
        let mut repo = ws.workdir().to_path_buf();
        repo.push(Repository::name());
        if !repo.exists() {
            fs::create_dir(&repo)
                .map_err(|e| Error::Repository(e.to_string()))?;
        }

        Database::init(repo.clone())?;
        Refs::init(repo.clone())?;
        LocalHead::init(repo)?;

        Ok(())
    }

    //refs may not contains oid for target branch name.
    pub fn read_head(&self) -> Result<Option<Oid>> {
        let head = self.local_head.get()?;

        let branch = if !head.is_branch() {
            return Ok(Some(head.oid()?.clone()));
        } else {
            head.branch()?
        };

        let oid = match self.refs.contains(branch) {
            true => Some(self.refs.get(branch)?),
            false => None
        };
        Ok(oid)
    }
}

impl IntoRev for Repository {
    fn into_rev(&self) -> Result<Rev> {
        let rev = match self.read_head()? {
            Some(oid) => {
                Revision::build(self.clone(), &oid)?
                    .into_rev()?
            },
            None => Rev::new(HashMap::new())
        };
        Ok(rev)
    }
}
