//! This module is responsible for
//! Load database/refs/index
//! and representing file/directory name for them.

pub mod database;
use database::*;

pub mod refs;
use refs::*;

pub mod workspace;
use workspace::*;

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
    pub path: path::PathBuf,
    pub workspace: Workspace,
    pub database: Database,
    pub refs: Refs,
}
impl Repository {
    pub fn build<P: AsRef<path::Path>>(workdir: P) -> io::Result<Self> {
        let workdir = workdir.as_ref().to_path_buf();

        let path = workdir.join(REPOSITORY);
        let workspace = Workspace::build(&workdir)?;
        let database = Database::new(path.join(DATABASE));
        let refs = Refs::new(path.join(REFS));

        let repo = Self {
            path,
            workspace,
            database,
            refs,
        };
        Ok(repo)
    }

    pub fn index_path(&self) -> path::PathBuf {
        self.path.join(INDEX)
    }
}
