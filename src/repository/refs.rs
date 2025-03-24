use std::{
    path::PathBuf,
    io,
};

pub const REFS: &str = "refs";
pub const HEADS: &str = "heads";

pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn new(repo: PathBuf) -> Self {
        let mut path = repo;
        path.push(REFS);

        Self {
            path
        }
    }
}
