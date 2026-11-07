use crate::prelude::*;
use std::path;

pub enum Ref {
    Oid(ObjectId),
    Sym(String),
}

pub struct Refs {
    path: path::PathBuf,
}
impl Refs {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }
}
