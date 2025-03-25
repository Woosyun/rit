pub mod head;
pub use head::*;

pub mod refs;
pub use refs::*;

pub mod database;
pub use database::*;


use std::path::PathBuf;
use crate::{
    workspace::Workspace,
    fs,
};

const RIT: &str = ".rit";

pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn build(ws: &Workspace) -> crate::Result<Self> {
        let mut path = ws.path.clone();
        path.push(RIT);

        if !path.exists() {
            return Err(crate::Error::Repository(".rit folder not found".into()));
        }

        let repo = Self {
            path
        };

        Ok(repo)
    }

    pub fn init(ws: &Workspace) -> crate::Result<&'static str> {
        let mut repo = ws.path.clone();
        repo.push(RIT);
        fs::create_dir(&repo)?;

        Database::init(repo)?;
        
        Ok("repository is created")
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
}
