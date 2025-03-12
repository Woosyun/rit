use crate::prelude::*;

pub struct Commit {
    parent: Option<Oid>,
    root_tree: Oid,
    message: String,
}

impl Commit {
    pub fn new(parent: Option<Oid>, root_tree: Oid, message: String) -> Self {
        Self {
            parent,
            root_tree,
            message,
        }
    }
}

impl Objectify for Commit {
    fn objectify(&self) -> String {
        let parent = match &self.parent {
            Some(oid) => oid.into_string(),
            None => "".to_string()
        };

        vec![
            parent,
            self.root_tree.into_string(),
            self.message.clone()
        ].join("\n")
    }
}
