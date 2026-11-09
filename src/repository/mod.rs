//! This module is responsible for
//! Load database/refs/index
//! and representing file/directory name for them.

pub mod database;
use database::*;

pub mod references;
use references::*;

pub mod workspace;
use workspace::*;

pub const REPOSITORY: &'static str = ".rit";
pub const DATABASE: &'static str = "objects";
pub const REFS: &'static str = "refs";
pub const INDEX: &'static str = "index";
pub const HEAD: &'static str = "HEAD";

use crate::prelude::*;
use std::io::Read;
use std::{fs, io, path};

//tothink:
//Index is only one representing actual data.
//Refs/Database/Workspace are abstracted representing logics.
//Now, how are you gonna setup interfaces?

pub struct Repository {
    pub path: path::PathBuf,
    pub workspace: Workspace,
    pub database: Database,
    pub references: References,
}
impl Repository {
    pub fn build<P: AsRef<path::Path>>(workdir: P) -> io::Result<Self> {
        let workdir = workdir.as_ref().to_path_buf();

        let path = workdir.join(REPOSITORY);
        let workspace = Workspace::build(&workdir)?;
        let database = Database::new(path.join(DATABASE));
        let references = References::new(path.join(REFS));

        let repo = Self {
            path,
            workspace,
            database,
            references,
        };
        Ok(repo)
    }

    pub fn read_index(&self) -> io::Result<Index> {
        let mut file = fs::File::open(self.path.join(INDEX))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Index::parse(&buf)
    }
    pub fn write_index(&self, index: Index) -> io::Result<()> {
        fs::write(self.path.join(INDEX), <Object as From<Index>>::from(index))
    }

    // Temporal reading head operation.
    // todo: Fix this to refer recursive ref
    pub fn read_head(&self) -> io::Result<Reference> {
        let head_path = RelativePath::build(HEAD)?;
        self.references.read(&head_path)
    }
}
