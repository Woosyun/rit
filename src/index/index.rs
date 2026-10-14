use crate::database;
use std::{collections, fs, os::unix::fs::MetadataExt, path};

pub struct Index {
    header: super::Header,
    entries: collections::HashMap<path::PathBuf, (fs::Metadata, database::Oid)>,
    hash: database::Oid,
}
impl Index {
    pub fn add(&mut self) {
        todo!("add command should consider entry name conflict")
    }
}
