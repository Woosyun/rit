use crate::{
    repository::database::Entry,
    workspace
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tree(Vec<Entry>);
impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self (entries)
    }
}

impl From<workspace::Tree> for Tree {
    fn from(item: workspace::Tree) -> Self {
        let entries = item
            .entries
            .into_iter()
            .map(|(_, entry)| {
                if let workspace::Entry::Entry(entry) = entry {
                    entry
                } else {
                    panic!("there is Entry::Tree in workspace::Tree");
                }
            })
            .collect::<Vec<_>>();

        Tree::new(entries)
    }
}

