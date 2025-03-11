use crate::prelude::*;

pub struct Commit {
    author: String,
    message: String,
    root_oid: Oid,
}

impl Commit {
    pub fn new(author: String, root_oid: Oid, message: String) -> Self {
        Self {
            author,
            root_oid,
            message,
        }
    }
}

impl Objectify for Commit {
    fn objectify(&self) -> String {
        format!("{}\n{}\n{}", self.author, self.root_oid.into_string(), self.message)
    }
}
