use crate::{database, lockfile, repository};
use std::io::Write;
use std::{io, path};

/// Name for "refs" folder consisted of branches.
pub const DIR_NAME: &'static str = "refs";

/// File name for "HEAD" file, representing current Head ref.
pub const HEAD_FILE_NAME: &'static str = "HEAD";

/// Responsible for handling refs, including head, branches, remotes.
pub struct Refs {
    path: path::PathBuf,
}
impl Refs {
    /// Returns [`Refs`] from repository directory
    pub fn new<P: AsRef<path::Path>>(repo_dir: P) -> Self {
        Self {
            path: repo_dir.as_ref().to_path_buf(),
        }
    }
    /// Read head.
    /*
     * hello
     */
    pub fn read_head(&self) -> io::Result<Ref> {
        let tmp = Ref::Oid(Default::default());
        Ok(tmp)
    }
    fn update_head_with_oid(&self, oid: database::Oid) -> io::Result<()> {
        //store as string(hex if oid)
        let target_path = self.path.join(HEAD_FILE_NAME);
        let mut lockfile = lockfile::LockFile::new(target_path);
        lockfile.write_all([oid.dir_name(), oid.file_name()].concat().as_bytes())?;
        lockfile.commit()
    }
    fn update_head_with_branch(&self, branch: &str) -> io::Result<()> {
        let target_path = self.path.join(HEAD_FILE_NAME);
        let mut lockfile = lockfile::LockFile::new(target_path);
        let content = ["ref: ", "refs/heads/", branch].join("");
        lockfile.write_all(content.as_bytes())?;
        lockfile.commit()
    }
    fn update_head<T: Into<Ref>>(&self, new_ref: T) -> io::Result<()> {
        let new_ref = new_ref.into();
        let mut target_ref = self.read_head()?;
        Ok(())
    }
}

pub enum Ref {
    Oid(database::Oid),
    SymRef(String),
}
impl Ref {
    pub fn oid(&self) -> Result<&database::Oid, String> {
        match self {
            Self::Oid(oid) => Ok(oid),
            Self::SymRef(_) => Err("expected oid, but got symref instead".into()),
        }
    }
}
impl From<database::Oid> for Ref {
    fn from(oid: database::Oid) -> Self {
        Ref::Oid(oid)
    }
}
impl From<String> for Ref {
    fn from(symref: String) -> Self {
        Ref::SymRef(symref)
    }
}
