pub mod head;
use head::*;

pub mod refs;
use refs::*;

pub mod database;
pub use database::*;

use crate::{
    prelude::*,
    fs,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
        let mut path = ws.path().to_path_buf();
        path.push(Repository::name());
        if !path.exists() {
            return Err(crate::Error::Repository(".rit folder not found".into()));
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
        let mut repo = ws.path().to_path_buf();
        repo.push(Repository::name());
        if !repo.exists() {
            fs::create_dir(&repo)?;
        }

        Database::init(repo.clone())?;
        Refs::init(repo.clone())?;
        LocalHead::init(repo)?;

        Ok(())
    }
}

impl IntoRev for Repository {
    fn into_rev(&self) -> Result<Rev> {
        let head = self.local_head.get()?;
        let branch = if !head.is_branch() {
            return Err(Error::Repository("cannot get Rev from non-branch head".to_string()));
        } else {
            head.branch()?
        };
        let rev = if self.refs.contains(branch) {
            let parent = self.refs.get(branch)?;
            Revision::build(self.clone(), &parent)?
                .into_rev()?
        } else {
            Rev::new(HashMap::new())
        };

        Ok(rev)
    }
}
