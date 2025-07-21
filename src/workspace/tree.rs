use std::{
    collections::{HashMap, VecDeque},
};
use crate::{
    repository,
    workspace::stat::*,
};

pub enum Entry {
    Tree(Tree),
    Entry(Box<dyn Stat>),
}

pub struct Tree{
    name: Name, 
    oid: Option<repository::Oid>,
    entries: HashMap<Name, Entry>,
}
impl Tree {
    pub fn new(name: String) -> Self {
        Self {
            name,
            oid: None,
            entries: HashMap::new(),
        }
    }
    pub fn entries(&self) -> &HashMap<Name, Entry> {
        &self.entries
    }

    pub fn add_entry(&mut self, components: &mut VecDeque<String>, new_entry: Box<dyn Stat>) {
        let Some(entry_name) = components.pop_front() else { return; };

        if components.is_empty() {
            self.entries.insert(entry_name, Entry::Entry(new_entry));
        } else {
            let Entry::Tree(tree) = self.entries.entry(entry_name.clone())
                .or_insert_with(|| Entry::Tree(Tree::new(entry_name)))
                else { return; };

            tree.add_entry(components, new_entry);
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

impl Stat for Tree {
    fn mode(&self) -> Mode {
        DIRECTORY_MODE
    }
    fn mtime(&self) -> Mtime {
        0
    }
    fn oid(&self) -> crate::Result<&repository::Oid> {
        match &self.oid {
            Some(oid) => Ok(oid),
            None => Err(crate::Error::Workspace("try to access oid of tree before it set".into()))
        }
    }
    fn set_oid(&mut self, oid: repository::Oid) {
        self.oid = Some(oid);
    }
    fn name(&self) -> &Name {
        &self.name
    }
}
