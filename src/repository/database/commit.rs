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
