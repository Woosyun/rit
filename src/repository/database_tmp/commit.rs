use std::hash::Hash;
use crate::prelude::*;

#[derive(Hash)]
pub struct Commit {
    message: String,
    author: String,
    commiter: String,
}

impl Commit {
    pub fn build(message: String, author: String, commiter: String) -> Self {
        Self {
            message,
            author,
            commiter
        }
    }
}

impl Objectify for Commit {
    fn objectify(&self) -> String {
        vec![
            self.message.clone(),
            self.author.clone(),
            self.commiter.clone()
        ]
            .join("\n")
    }
}
impl CalculateHash for Commit {}
