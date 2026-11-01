//! This module is responsible for
//! Load database/refs/index
//! and representing file/directory name for them.

pub mod database;
pub use database::*;

pub mod workspace;
pub use workspace::*;

pub mod refs;
pub use refs::*;

pub const REPOSITORY: &'static str = ".rit";
pub const DATABASE: &'static str = "objects";
pub const REFS: &'static str = "refs";
pub const INDEX: &'static str = "index";
pub const HEAD: &'static str = "HEAD";

use crate::index;
use std::{error, path};

//tothink:
//Index is only one representing actual data.
//Refs/Database/Workspace are abstracted representing logics.
//Now, how are you gonna setup interfaces?

pub struct Repository {
    pub repodir: path::PathBuf,
    pub workspace: Workspace,
    pub database: Database,
    pub index: index::Index,
    pub refs: Refs,
}
impl Repository {
    pub fn build<P: AsRef<path::Path>>(workdir: P) -> Result<Self, Box<dyn error::Error>> {
        let workdir = workdir.as_ref().to_path_buf();

        let repodir = workdir.join(REPOSITORY);
        let workspace = Workspace::new(&workdir);
        let database = Database::new(repodir.join(DATABASE));
        let index = todo!("load index");
        let refs = Refs::new(repodir.join(REFS));

        let repo = Self {
            repodir,
            workspace,
            database,
            index,
            refs,
        };
        Ok(repo)
    }
}
