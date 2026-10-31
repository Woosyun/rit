use std::{ops, path};

#[derive(Eq, PartialEq, Hash)]
pub struct RelativePath(path::PathBuf);
impl RelativePath {
    pub fn new(path: path::PathBuf) -> Self {
        Self(path)
    }
}
//todo: implement DeRef
impl ops::Deref for RelativePath {
    type Target = path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<path::PathBuf> for RelativePath {
    fn from(item: path::PathBuf) -> Self {
        Self(item)
    }
}
impl From<RelativePath> for path::PathBuf {
    fn from(item: RelativePath) -> path::PathBuf {
        item.0
    }
}
