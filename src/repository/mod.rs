//! This module is responsible for
//! Load database/refs/index
//! and representing file/directory name for them.

pub mod database;
use database::*;

// pub mod references;
// use references::*;

pub mod workspace;
use workspace::*;

pub const REPOSITORY: &str = ".git";
pub const DATABASE: &str = "objects";
pub const REFS: &str = "refs";
pub const HEADS: &str = "heads";
pub const INDEX: &str = "index";
pub const HEAD: &str = "HEAD";

use crate::prelude::*;
use std::io::Read;
use std::{fs, io, path};

pub struct Repository {
    pub path: path::PathBuf,
    pub workspace: Workspace,
    pub database: Database,
}
impl Repository {
    pub fn build<P: AsRef<path::Path>>(workdir: P) -> io::Result<Self> {
        let workdir = workdir.as_ref().canonicalize()?;

        let path = workdir.join(REPOSITORY);
        let workspace = Workspace::new(&workdir);
        let database = Database::new(path.join(DATABASE));

        let repo = Self {
            path,
            workspace,
            database,
        };
        Ok(repo)
    }

    pub fn read_index(&self) -> io::Result<Index> {
        let mut file = fs::File::open(self.path.join(INDEX))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Index::parse(&buf)
    }
    pub fn write_index(&self, index: Index) -> io::Result<()> {
        fs::write(self.path.join(INDEX), <Vec<u8> as From<Index>>::from(index))
    }

    pub fn read_head(&self) -> io::Result<Reference> {
        self.read_reference(&HEAD.into())
    }

    // tothink:
    // Why Repository is responsible for parsing raw reference to Reference?
    pub fn read_reference(&self, dest_relative_path: &RelativePath) -> io::Result<Reference> {
        let path = self.path.join(dest_relative_path);
        let ref_str = fs::read_to_string(path)?;
        let reference = if let Some(sym_ref) = ref_str.strip_prefix(SYM_REF_PREFIX) {
            Reference::Sym(sym_ref.into())
        } else {
            let oid = <ObjectId as TryFrom<&str>>::try_from(&ref_str)?;
            Reference::Oid(oid)
        };

        Ok(reference)
    }
    pub fn write_reference(
        &self,
        dest_relative_path: &RelativePath,
        reference: &Reference,
    ) -> io::Result<()> {
        let path = self.path.join(dest_relative_path);
        let ref_str = reference.to_string();
        fs::write(path, ref_str)
    }
}
