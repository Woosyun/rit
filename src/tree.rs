use crate::prelude::*;
use std::{
    collections::HashMap,
    path::PathBuf,
    io,
};

enum Entry {
    Tree(Tree),
    Blob(Oid),
}
pub struct Tree {
    pub oid: Option<Oid>,
    children: HashMap<String, Entry>
}
impl Tree {
    pub fn new() -> Self {
        Self {
            oid: None,
            children: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, mut ancestors: Vec<PathBuf>, oid: Oid) {
        let file_name = ancestors.pop().unwrap()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string();

        if ancestors.is_empty() {
            let _ = self.children.insert(file_name, Entry::Blob(oid));
            return;
        }

        if let Some(tree) = self.children.get_mut(&file_name) {
            if let Entry::Tree(tree) = tree {
                tree.add_entry(ancestors, oid);
            }
        } else {
            let mut tree = Tree::new();
            tree.add_entry(ancestors, oid);
            self.children.insert(file_name, Entry::Tree(tree));
        }
    }

    pub fn traverse_mut<F: Fn(&mut Tree) -> io::Result<()> + Copy>(&mut self, f: F) -> io::Result<()> {
        for (_, entry) in self.children.iter_mut() {
            match entry {
                Entry::Tree(tree) => tree.traverse_mut(f)?,
                _ => (),
            }
        }

        f(self)
    }
}

impl Objectify for Tree {
    fn objectify(&self) -> String {
        self.children.iter()
            .map(|(name, entry)| {
                match entry {
                    Entry::Tree(tree) => {
                        if let Some(oid) = &tree.oid {
                            format!("40000 tree {} {}", oid.into_string(), name)
                        } else {
                            //this code should not run?!
                            format!("")
                        }
                    }
                    Entry::Blob(oid) => format!("100644 blob {} {}", oid.into_string(), name),
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
