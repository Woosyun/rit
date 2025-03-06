use std::collections::HashMap;
use crate::prelude::*;

#[derive(Clone)]
pub enum TreeEntry {
    Tree(Tree),
    Entry(Entry),
}
#[derive(Clone)]
pub struct Tree {
    oid: Option<Hash256>,
    children: HashMap<String, TreeEntry>
}
impl Tree {
    pub fn traverse<F: Fn(&Tree) + Copy>(&self, f: F) {
        let _ = self.children
            .iter()
            .map(|(_, child)| {
                match child {
                    TreeEntry::Tree(tree) => tree.traverse(f),
                    TreeEntry::Entry(_) => (),
                }
            });

        f(self)
    }
}
impl From<Tree> for String {
    fn from(item: Tree) -> String {
        format!("{} tree {} {}", 
    }
}
impl Objectify for Tree {
    fn objectify(&self) -> String {
        // don't make new entry?
        self.children.clone()
            .into_iter()
            .map(|(name, child)| {
                match child {
                    TreeEntry::Tree(ref tree) => tree.into(),
                    TreeEntry::Entry(entry) => entry.into(),
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
    fn set_oid(&mut self, oid: Hash256) {
        self.oid = Some(oid);
    }
    fn get_oid(&self) -> Option<&Hash256> {
        self.oid.as_ref()
    }
}

#[derive(Clone)]
pub struct Entry {
    kind: EntryKind,
    oid: Hash256,
    name: String,
}
impl Entry {
    pub fn new(kind: EntryKind, oid: Hash256, name: String) -> Self {
        Self {
            kind,
            oid,
            name,
        }
    }
    fn mode(&self) -> &str {
        match self.kind {
            EntryKind::Tree => "100644",
            EntryKind::Blob => "40000",
        }
    }
    fn kind(&self) -> &str {
        match self.kind {
            EntryKind::Tree => "tree",
            EntryKind::Blob => "blob",
        }
    }
    fn oid(&self) -> &str {
        self.oid.clone().into()
    }
    fn name(&self) -> &str {
        self.name.as_ref()
    }
}
impl From<Entry> for String {
    fn from(item: Entry) -> String {
        format!("{} {} {} {}", item.mode(), item.kind(), item.oid(), item.name())
    }
}

#[derive(Clone)]
pub enum EntryKind {
    Blob,
    Tree,
}



