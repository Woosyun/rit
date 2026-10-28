use crate::{database, lockfile};
#[allow(unused)]
use std::io::{Read, Seek, SeekFrom};
use std::{collections, fs, io, path};

const FILE_NAME: &'static str = "index";

/// # Index
/// Responsible for dealing with 'Index'
#[derive(Default)]
pub struct Index {
    lockfile: Option<lockfile::LockFile>,
    header: super::Header,
    //todo: use stage
    entries: collections::HashMap<path::PathBuf, super::Entry>,
    hash: database::Oid,
}
impl Index {
    pub fn new<P: AsRef<path::Path>>(repo: P) -> Self {
        Self {
            lockfile: None,
            header: Default::default(),
            entries: collections::HashMap::new(),
            hash: Default::default(),
        }
    }

    //tothink: is AsRef<Path> always equivalent PathBuf?
    pub fn insert(
        &mut self,
        relative_path: path::PathBuf,
        metadata: fs::Metadata,
        oid: database::Oid,
    ) {
        let filename = if let Some(osstr) = relative_path.file_name() {
            if let Some(str) = osstr.to_str() {
                str.to_string()
            } else {
                //tothink: are these allowable behaviour?
                return;
            }
        } else {
            return;
        };
        let entry = super::Entry::new(metadata, oid, filename);
        let _ = self.entries.insert(relative_path, entry);
    }

    pub fn entries(&self) -> &collections::HashMap<path::PathBuf, super::Entry> {
        &self.entries
    }

    pub fn commit(self) -> io::Result<()> {
        //currently, every request fall into error
        let Some(_lockfile) = self.lockfile else {
            return Err(io::Error::from(io::ErrorKind::PermissionDenied));
        };
        todo!("write to file");
    }
}
