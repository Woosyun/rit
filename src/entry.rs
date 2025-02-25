use std::path::PathBuf;
use crate::hash256::Hash256;

pub struct Entry {
    pub path: PathBuf,
    pub oid: Hash256,
}
impl Entry {
    pub fn build(path: PathBuf, oid: Hash256) -> Self {
        Self {
            path,
            oid,
        }
    }
}
