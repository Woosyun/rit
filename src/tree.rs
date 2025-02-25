use crate::entry::Entry;
use std::path::{Path, PathBuf};

pub enum TreeEntry {
    Tree(Tree),
    Entry(Entry),
}

pub struct Tree(Vec<(String, TreeEntry)>);
impl Tree {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add_entry(&mut self, mut ancestors: Vec<&Path>, entry: Entry) {
        if let Some(next) = ancestors.pop() {
            // there is at least one parent path
            let next = next.file_name().unwrap().to_str().unwrap();
            let elem = self.0
                .iter_mut()
                .find(|(name, _)| name == next);

            if let Some(elem) = elem {
                // TreeEntry is already exist
                let (_, tree_entry) = elem;
                match tree_entry {
                    TreeEntry::Tree(tree) => tree.add_entry(ancestors, entry),
                    _ => (), // shouldn't this return error?
                }
            } else {
                // create new TreeEntry
                // todo: implement sort?
                let mut new_tree = Tree::new();
                new_tree.add_entry(ancestors, entry);
                self.0.push((next.to_string(), TreeEntry::Tree(new_tree)));
            }
        } else {
            // put entry to self
            let file_name = entry.path.file_name().unwrap().to_str().unwrap().to_string();
            self.0.push((file_name, TreeEntry::Entry(entry)));
        }
    }

    pub fn traverse<F: Fn(&Tree) + Copy>(&self, f: F) {
        self.0.iter()
            .map(|tree_entry| {
                match tree_entry {
                    (_, TreeEntry::Tree(tree)) => tree.traverse(f),
                    _ => ()
                }
            });

        f(self)
    }
}
