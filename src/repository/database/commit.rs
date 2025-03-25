use crate::{
    repository::database::Oid,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Commit {
    parent: Option<Oid>,
    root_tree: Oid,
    message: String,
    //commiter: String,
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
