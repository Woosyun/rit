pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod revision;
pub use revision::*;

use std::ops;

/// Object represents Blob/Tree/Commit.
pub struct Object(Vec<u8>);
impl Object {
    pub fn new(content: Vec<u8>) -> Self {
        Self(content)
    }
}
impl ops::Deref for Object {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub type ObjectId = [u8; 20];

/*
#[derive(Clone, Default)]
pub struct ObjectId(pub [u8; 20]);
impl ObjectId {
    pub fn new(oid: [u8; 20]) -> Self {
        Self(oid)
    }
}
*/
