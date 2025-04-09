pub mod head;
use head::*;

pub mod refs;
use refs::*;

pub mod database;
pub use database::*;

use crate::{
    workspace::Workspace,
    fs,
};
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    pub db: Database,
    pub head: Head,
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
        let head = Head::build(path.clone());
        let refs = Refs::build(path.clone())?;

        let repo = Self {
            db,
            head,
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
        Refs::init(repo)?;

        Ok(())
    }
    pub fn get_head(&self) -> crate::Result<Option<Oid>> {
        if let Some(branch) = self.head.get()? {
            let oid = self.refs.get(&branch)?;
            Ok(Some(oid))
        } else {
            Ok(None)
        }
    }
    pub fn set_head(&self, oid: &Oid) -> crate::Result<()> {
        if let Some(branch) = self.head.get()? {
            self.refs.set(&branch, oid)?;
        } else {
            self.refs.set("main", oid)?;
            self.head.set("main")?;
        }
        Ok(())
    }
}
