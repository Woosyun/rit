use crate::{
    repository::Oid,
};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    parents: Vec<Oid>, // zero or one oid represent normal commit, two represents merge commit
    root: Oid,
    message: String,
    //commiter: String,
}

impl Commit {
    pub fn new(parents: Vec<Oid>, root: Oid, message: String) -> Self {
        Self {
            parents,
            root,
            message,
        }
    }
    pub fn root(&self) -> &Oid {
        &self.root
    }
    pub fn parents(&self) -> &Vec<Oid> {
        &self.parents
    }
    pub fn message(&self) -> &str {
        &self.message
    }
    pub fn is_merged(&self) -> bool {
        self.parents.len() == 2
    }
}
