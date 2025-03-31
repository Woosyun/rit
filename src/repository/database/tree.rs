use crate::{
    repository::Entry,
    workspace
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tree(Vec<Entry>);
impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self (entries)
    }
    pub fn entries(&self) -> &Vec<Entry> {
        &self.0
    }
}

// todo: Maybe this is unnecesary code. Remove it.
impl From<workspace::Tree> for Tree {
    fn from(item: workspace::Tree) -> Self {
        let entries = item
            .entries
            .into_iter()
            .map(|(_, entry)| {
                match entry {
                    workspace::Entry::Entry(entry) => entry,
                    _ => panic!("there is Entry::Tree in workspace::Tree"),
                }
            })
            .collect::<Vec<_>>();

        Tree::new(entries)
    }
}
