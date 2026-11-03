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

use crate::prelude::*;
use std::{fs, io, path};

//tothink:
//Index is only one representing actual data.
//Refs/Database/Workspace are abstracted representing logics.
//Now, how are you gonna setup interfaces?

pub struct Repository {
    pub repodir: path::PathBuf,
    pub workspace: Workspace,
    pub database: Database,
    pub index: Index,
    pub refs: Refs,
}
impl Repository {
    pub fn build<P: AsRef<path::Path>>(workdir: P) -> io::Result<Self> {
        let workdir = workdir.as_ref().to_path_buf();

        let repodir = workdir.join(REPOSITORY);
        let workspace = Workspace::new(&workdir);
        let database = Database::new(repodir.join(DATABASE));
        let index = load_index(repodir.join(INDEX))?;
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

fn load_index<P: AsRef<path::Path>>(path: P) -> io::Result<Index> {
    let index_raw = fs::read(path)?;
    Index::parse(&index_raw)
}
