use crate::prelude::*;
use std::{collections, fs, os::unix::fs::MetadataExt};

/// # Index
pub struct Index {
    header: Header,
    //todo: use stage
    entries: collections::HashMap<RelativePath, Entry>,
    hash: ObjectId,
}
impl Index {
    pub fn entries(&self) -> &collections::HashMap<RelativePath, Entry> {
        &self.entries
    }
}

type ToBeEntry = (RelativePath, fs::Metadata, ObjectId);
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

struct Header {
    signature: [char; 4],
    version: u32,
    number_of_entries: u32,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            signature: ['D', 'I', 'R', 'C'],
            version: 2,
            number_of_entries: 0,
        }
    }
}

struct Entry {
    ctime: u32,
    ctime_nano: u32,
    mtime: u32,
    mtime_nano: u32,
    dev: u32,
    ino: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    oid: ObjectId,
    flags: Flags,
    //(optional?) extened_flags: u16,
    file_name: String,
}
impl Entry {
    fn new(stat: fs::Metadata, oid: ObjectId, file_name: String) -> Self {
        Self {
            ctime: stat.ctime() as u32,
            ctime_nano: stat.ctime_nsec() as u32,
            mtime: stat.mtime() as u32,
            mtime_nano: stat.mtime_nsec() as u32,
            dev: stat.dev() as u32,
            ino: stat.ino() as u32,
            mode: stat.mode() as u32,
            uid: stat.uid() as u32,
            gid: stat.gid() as u32,
            file_size: stat.size() as u32,
            oid,
            flags: Default::default(),
            file_name,
        }
    }
    pub fn mode(&self) -> &u32 {
        &self.mode
    }
    pub fn oid(&self) -> &ObjectId {
        &self.oid
    }
}

/// 1-bit assume-valid flag
/// 1-bit extended flag (must be zero in version 2)
/// 2-bit stage (used during merge)
/// 12-bit length of file name, max(0xfff, len(file name))
#[derive(Default)]
struct Flags(u16);
/*
impl Default for Flags {
    fn default() -> Flags {
        let f: u16 = 0b_0000_0000_0000_0000;
        Flags(f)
    }
}
*/
