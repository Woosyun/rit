use std::ops::Deref;
use std::{io, ops, path};

pub fn is_path_relative<B, T>(base: B, target: T) -> io::Result<bool>
where
    B: AsRef<path::Path>,
    T: AsRef<path::Path>,
{
    let base_abs = base.as_ref().canonicalize()?;
    let target_abs = target.as_ref().canonicalize()?;
    let path = target_abs.strip_prefix(&base_abs).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "target path is not relative to base path",
        )
    })?;

    Ok(path != target.as_ref())
}
pub fn get_relative_path<B, T>(base: B, target: T) -> io::Result<RelativePath>
where
    B: AsRef<path::Path>,
    T: AsRef<path::Path>,
{
    let base = base.as_ref().canonicalize()?;
    let target = target.as_ref().canonicalize()?;
    let path = target
        .strip_prefix(&base)
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "target path is not relative to base path",
            )
        })?
        .to_path_buf();

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

use std::fmt;
impl fmt::Display for RelativePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.to_str().unwrap_or_default();
        write!(f, "{}", s)
    }
}
