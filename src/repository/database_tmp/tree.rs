use crate::prelude::*;
use std::collections::HashMap;

pub struct Tree {
    pub oid: Option<Hash256>,
    pub children: HashMap<String, TreeEntry>,
}
impl Tree {
    pub fn new() -> Self {
        Self {
            oid: None,
            children: HashMap::new(),
        }
    }
    #[allow(unused)]
    pub fn add_entry(&mut self, ancestors: Vec<String>, entry: Entry) {
    }
    pub fn set_oid(&mut self) -> io::Result<()> {
        let oid = self.calculate_hash()?;
        self.oid = Some(oid);
        Ok(())
    }
}
impl Objectify for Tree {
    fn objectify(&self) -> String {
        self.children
            .clone()
            .into_iter()
            .map(|(name, child)| {
                match child {
                    TreeEntry::Tree(tree) => Entry::new(EntryKind::Tree, tree.oid.unwrap(), name),
                    TreeEntry::Entry(entry) => entry,
                }.into()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
impl CalculateHash for Tree {}

#[derive(Clone)]
pub enum TreeEntry {
    Tree(Tree),
    Entry(Entry),
}

pub enum EntryKind {
    Blob,
    Tree,
}

pub struct Entry {
    pub kind: EntryKind,
    pub oid: Hash256,
    pub name: String,
}
impl Entry {
    pub fn new(kind: EntryKind, oid: Hash256, name: String) -> Self {
        Self {
            kind,
            oid,
            name,
        }
    }
    pub fn mode(&self) -> &'static str {
        match self.kind {
            EntryKind::Blob => "100644",
            EntryKind::Tree => "40000",
        }
    }
    pub fn kind(&self) -> &'static str {
        match self.kind {
            EntryKind::Blob => "blob",
            EntryKind::Tree => "tree",
        }
    }
    pub fn oid(&self) -> String {
        self.oid.clone().into()
    }
}
impl From<Entry> for String {
    fn from(entry: Entry) -> String {
        format!("{} {} {} {}", entry.mode(), entry.kind(), entry.oid(), entry.name)
    }
}
