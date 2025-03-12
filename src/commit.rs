use crate::prelude::*;

pub struct Commit {
    parent_oid: Option<Oid>,
    root_oid: Oid,
    message: String,
}

impl Commit {
    pub fn new(parent_oid: Option<Oid>, root_oid: Oid, message: String) -> Self {
        Self {
            parent_oid,
            root_oid,
            message,
        }
    }
}

impl Objectify for Commit {
    fn objectify(&self) -> String {
        let parent = match &self.parent_oid {
            Some(oid) => oid.decode(),
            None => "".to_string()
        };

        vec![
            parent,
            self.root_oid.decode(),
            self.message.clone()
        ].join("\n")
    }
}
