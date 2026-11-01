//! This module is responsible for
//! Load database/refs/index
//! and representing file/directory name for them.

pub const REPOSITORY: &'static str = ".rit";
pub const DATABASE: &'static str = "objects";
pub const REFS: &'static str = "refs";
pub const INDEX: &'static str = "index";
pub const HEAD: &'static str = "HEAD";

use std::path;

//tothink:
//Index is only one representing actual data.
//Refs/Database/Workspace are abstracted representing logics.
//Now, how are you gonna setup interfaces?

pub struct Repository {
    workdir: path::PathBuf,
    repodir: path::PathBuf,
    /*
    pub database: database::Database,
    pub index: index::Index,
    pub refs: refs::Refs,
    */
}
