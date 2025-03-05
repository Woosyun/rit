use std::{
    hash::Hash,
    path::{PathBuf, Path},
};
use crate::prelude::*;

#[derive(Hash)]
pub struct Entry {
    pub kind: EntryKind,
    pub path: PathBuf,
    pub hash: Option<Hash256>,
    pub children: Vec<Entry>
}
#[derive(Hash)]
pub enum EntryKind {
    Blob,
    Tree,
}

impl Entry {
    pub fn build_tree(path: PathBuf) -> Self {
        Self {
            kind: EntryKind::Tree,
            path,
            hash: None,
            children: vec![]
        }
    }
    pub fn build_blob(path: PathBuf, hash: Option<Hash256>) -> Self {
        Self {
            kind: EntryKind::Blob,
            path,
            hash,
            children: vec![]
        }
    }

    pub fn add_entry(&mut self, mut ancestors: Vec<&Path>, entry: Entry) {
        if ancestors.len() == 0 {
            //add entry
            self.children.push(entry);
            return;
        }

        let ancestor = ancestors.pop().unwrap();
        let ancestor = ancestor
            .file_name().unwrap()
            .to_str().unwrap();
        for e in &mut self.children {
            let e_name = e.path
                .file_name().unwrap()
                .to_str().unwrap();
            if e_name == ancestor {
                return e.add_entry(ancestors, entry);
            }
        }

        //create new tree
        let mut new_path = self.path.clone();
        new_path.push(ancestor);
        let mut new_tree = Entry::build_tree(new_path);
        new_tree.add_entry(ancestors, entry);
        self.children.push(new_tree);
    }

    pub fn traverse<F: Fn(&Entry) + Copy>(&self, f: F) {
        for entry in &self.children {
            match entry.kind {
                EntryKind::Tree => entry.traverse(f),
                _ => ()
            }
        }

        f(self)
    }
    pub fn traverse_mut<F: Fn(&mut Entry) + Copy>(&mut self, f: F) {
        for entry in &mut self.children {
            match entry.kind {
                EntryKind::Tree => entry.traverse_mut(f),
                _ => ()
            }
        }

        f(self)
    }
}

impl Objectify for Entry {
    fn objectify(&self) -> String {
        self.children
            .iter()
            .map(|entry| {
                let (mode, kind) = match entry.kind {
                    EntryKind::Tree => ("40000", "tree"),
                    EntryKind::Blob => ("10644", "blob")
                };
                let hash: String = entry.hash.clone()
                    .unwrap().into();
                let name = entry.path
                    .file_name().unwrap()
                    .to_str().unwrap();

                format!("{mode} {kind} {hash} {name}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
impl CalculateHash for Entry {}
