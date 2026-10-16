use crate::database;
//tothink: os::unix??
use std::{fs, os::unix::fs::MetadataExt};

pub struct Entry {
    ctime: u32,
    ctime_nano: u32,
    mtime: u32,
    mtime_nano: u32,
    dev: u32,
    ino: u32,
    //todo: define mode
    mode: u16,
    uid: u32,
    gid: u32,
    file_size: u32,
    oid: database::Oid,
    flags: Flags,
    //extened_flags: u16,
    file_name: String,
}
impl Entry {
    pub fn new(stat: fs::Metadata, oid: database::Oid, file_name: String) -> Self {
        Self {
            ctime: stat.ctime() as u32,
            ctime_nano: stat.ctime_nsec() as u32,
            mtime: stat.mtime() as u32,
            mtime_nano: stat.mtime_nsec() as u32,
            dev: stat.dev() as u32,
            ino: stat.ino() as u32,
            mode: stat.mode() as u16,
            uid: stat.uid() as u32,
            gid: stat.gid() as u32,
            file_size: stat.size() as u32,
            oid,
            flags: Default::default(),
            file_name,
        }
    }
    pub fn mode(&self) -> &u16 {
        &self.mode
    }
    pub fn oid(&self) -> &database::Oid {
        &self.oid
    }
}

/// 1-bit assume-valid flag
/// 1-bit extended flag (must be zero in version 2)
/// 2-bit stage (used during merge)
/// 12-bit length of file name, max(0xfff, len(file name))
pub struct Flags(u16);
impl Default for Flags {
    fn default() -> Flags {
        let f: u16 = 0b_0000_0000_0000_0000;
        Flags(f)
    }
}
