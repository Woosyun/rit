use std::ops::Deref;
use std::{error, io, ops, path};

pub fn get_relative_path<P: AsRef<path::Path>>(
    base: P,
    target: P,
) -> Result<RelativePath, Box<dyn error::Error>> {
    let path = target
        .as_ref()
        .canonicalize()?
        .strip_prefix(base.as_ref())?
        .to_path_buf();

    RelativePath::build(path).map_err(Into::into)
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct RelativePath(path::PathBuf);
impl RelativePath {
    // This method is being used by "HEAD" constant.
    /*
    pub fn new(path: path::PathBuf) -> Self {
        Self(path)
    }
    */

    pub fn build<P: AsRef<path::Path>>(p: P) -> io::Result<Self> {
        //todo: check whether path is canonicalized or not.
        let p = p.as_ref();
        if !p.is_relative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "path is not relative",
            ));
        }
        let rp = RelativePath(p.to_path_buf());
        Ok(rp)
    }
}

impl TryFrom<path::PathBuf> for RelativePath {
    type Error = io::Error;
    fn try_from(p: path::PathBuf) -> Result<Self, Self::Error> {
        Self::build(p)
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
