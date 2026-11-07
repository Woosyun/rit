use std::{ops, path};

pub struct RelativePath(path::PathBuf);
impl RelativePath {
    pub fn new(path: path::PathBuf) -> Self {
        Self(path)
    }
}

impl ops::Deref for RelativePath {
    type Target = path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RelativePath {
    fn from(str: String) -> Self {
        Self::new(str.into())
    }
}
