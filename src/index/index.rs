use super::entry::Entry;
use crate::prelude::*;
use std::{collections, fs};

const FILE_NAME: &'static str = "index";

/// # Index
pub struct Index {
    header: super::Header,
    //todo: use stage
    entries: collections::HashMap<RelativePath, super::Entry>,
    hash: database::Oid,
}
impl Index {
    pub fn entries(&self) -> &collections::HashMap<RelativePath, super::Entry> {
        &self.entries
    }
}

type ToBeEntry = (RelativePath, fs::Metadata, database::Oid);
impl FromIterator<ToBeEntry> for Index {
    fn from_iter<I: IntoIterator<Item = ToBeEntry>>(iter: I) -> Self {
        let m = iter
            .into_iter()
            .map(|(relative_path, metadata, oid)| {
                let file_name = relative_path
                    .file_name()
                    .expect("entry should be normal")
                    .to_str()
                    .expect("this is valid utf-8(?)")
                    .to_string();

                (relative_path, Entry::new(metadata, oid, file_name))
            })
            .collect::<collections::HashMap<RelativePath, Entry>>();

        Index {
            header: Default::default(),
            entries: m,
            hash: Default::default(), // todo: make it being updated along the iterating.
        }
    }
}
/*
impl Index {
    pub fn new() -> Self {
        Self {
            header: Default::default(),
            entries: collections::HashMap::new(),
            hash: Default::default(),
        }
    }

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
*/
