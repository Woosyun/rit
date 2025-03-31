pub mod head;
pub use head::*;

pub mod refs;
pub use refs::*;

pub mod database;
pub use database::*;

pub mod ignore;
pub use ignore::*;


use std::path::PathBuf;
use crate::{
    workspace::Workspace,
    fs,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn name() -> &'static str {
        ".rit"
    }
    pub fn build(ws: &Workspace) -> crate::Result<Self> {
        let mut path = ws.path.clone();
        path.push(Repository::name());

        if !path.exists() {
            return Err(crate::Error::Repository(".rit folder not found".into()));
        }

        let repo = Self {
            path
        };

        Ok(repo)
    }

    pub fn init(ws: &Workspace) -> crate::Result<()> {
        let mut repo = ws.path.clone();
        repo.push(Repository::name());
        if !repo.exists() {
            fs::create_dir(&repo)?;
        }

        Database::init(repo)
    }

    pub fn get_database(&self) -> crate::Result<Database> {
        Database::build(self.path.clone())
    }
    pub fn get_head(&self) -> Head {
        Head::new(self.path.clone())
    }
    pub fn get_refs(&self) -> Refs {
        Refs::new(self.path.clone())
    }
    pub fn get_ignore(&self) -> crate::Result<Ignore> {
        Ignore::build(self.path.clone())
    }
}
