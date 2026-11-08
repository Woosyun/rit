use std::ops::Deref;
use std::{io, ops, path};

#[derive(Clone)]
pub struct RelativePath(path::PathBuf);
impl RelativePath {
    // This method is being used by "HEAD" constant.
    pub fn new(path: path::PathBuf) -> Self {
        Self(path)
    }

    pub fn build<P: AsRef<path::Path>>(p: P) -> io::Result<Self> {
        //todo: check whether path is canonicalized or not.
        let p = p.as_ref();
        if !p.is_relative() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "path is not relative",
            ));
        }
        let rp = Self::new(p.to_path_buf());
        Ok(rp)
    }
}

impl TryFrom<path::PathBuf> for RelativePath {
    type Error = io::Error;

    fn try_from(p: path::PathBuf) -> Result<Self, Self::Error> {
        Self::build(p)
    }
}

impl<P> AsRef<P> for RelativePath
where
    P: ?Sized,
    <RelativePath as ops::Deref>::Target: AsRef<P>,
{
    fn as_ref(&self) -> &P {
        self.deref().as_ref()
    }
}

impl ops::Deref for RelativePath {
    type Target = path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
