use crate::{
    repository::Oid,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    parent: Option<Oid>,
    root: Oid,
    message: String,
    //commiter: String,
}

impl Commit {
    pub fn new(parent: Option<Oid>, root: Oid, message: String) -> Self {
        Self {
            parent,
            root,
            message,
        }
    }
    pub fn root(&self) -> &Oid {
        &self.root
    }
    pub fn parent(&self) -> &Option<Oid> {
        &self.parent
    }
}
