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
impl Objectify for Tree {
    fn objectify<'a>(&'a self) -> &'a str {
        &self.children
            .iter()
            .map(|(name, tree_entry)| {
                let entry = match tree_entry {
                    TreeEntry::Tree(tree) => Entry::new(EntryKind::Tree, tree.oid.unwrap(), name.clone()),
                    TreeEntry::Entry(entry) => entry,
                };

                entry.into()
            })
            .collect::<Vec<_>>()
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



