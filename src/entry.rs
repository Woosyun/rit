use crate::prelude::*;

pub struct Entry {
    kind: EntryKind,
    oid: Oid,
    name: String,
}
pub enum EntryKind {
    Tree,
    Blob,
}
impl Entry {
    pub fn from_blob(oid: Oid, name: String) -> Entry {
        Self {
            kind: EntryKind::Blob,
            oid,
            name,
        }
    }
    pub fn from_tree(oid: Oid, name: String) -> Entry {
        Self {
            kind: EntryKind::Tree,
            oid,
            name,
        }
    }
}

impl From<&Entry> for String {
    fn from(item: &Entry) -> String {
        let oid = &item.oid;
        let oid: String = oid.into();
        let name = item.name.clone();

        match item.kind {
            EntryKind::Tree => format!("40000 tree {} {}", oid, name),
            EntryKind::Blob => format!("100644 blob {} {}", oid, name),
        }
    }
}
