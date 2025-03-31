use std::{
    collections::HashMap,
};
use serde::{Serialize, Deserialize};
use crate::repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entry {
    Tree(Tree),
    Entry(repository::Entry),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tree{
    pub oid: Option<repository::Oid>,
    pub entries: HashMap<String, Entry>,
}
impl Tree {
    pub fn new() -> Self {
        Self {
            oid: None,
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, ancestors: &mut Vec<String>, entry: repository::Entry) {
        let file_name = ancestors.pop().unwrap();

        if ancestors.is_empty() {
            let _ = self.entries.insert(file_name, Entry::Entry(entry));
            return;
        }

        if let Some(tree) = self.entries.get_mut(&file_name) {
            if let Entry::Tree(tree) = tree {
                tree.add_entry(ancestors, entry);
            }
        } else {
            let mut tree = Tree::new();
            tree.add_entry(ancestors, entry);
            self.entries.insert(file_name, Entry::Tree(tree));
        }
    }

    // depending on the return type of traverse function,
    // Tree might not needs to store oid, name, or other informations
    // those needed to create entry, of database::Tree.
    pub fn traverse_mut<F: Fn(&mut Tree) -> crate::Result<()> + Copy>(&mut self, f: F) -> crate::Result<()> {
        for (_, entry) in self.entries.iter_mut() {
            if let Entry::Tree(tree) = entry {
                let _ = tree.traverse_mut(f)?;
            }
        }

        f(self)
    }
}

