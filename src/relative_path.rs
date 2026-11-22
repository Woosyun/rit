use std::ops::Deref;
use std::{error, ops, path};

pub fn get_relative_path<B, T>(base: B, target: T) -> Result<RelativePath, Box<dyn error::Error>>
where
    B: AsRef<path::Path>,
    T: AsRef<path::Path>,
{
    let base = base.as_ref().canonicalize()?;
    let target = target.as_ref().canonicalize()?;
    let path = target.strip_prefix(&base)?.to_path_buf();

    Ok(RelativePath::new(path))
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct RelativePath(path::PathBuf);
impl RelativePath {
    // This method is used to convert path
    // given by tree, index, etc.
    pub fn new(path: path::PathBuf) -> Self {
        Self(path)
    }
}

impl AsRef<path::Path> for RelativePath {
    fn as_ref(&self) -> &path::Path {
        self.deref().as_ref()
    }
}

impl ops::Deref for RelativePath {
    type Target = path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
