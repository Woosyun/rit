pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod commit;
pub use commit::*;

use std::ops;

/// Object represents Blob/Tree/Commit.
pub struct Object(Vec<u8>);
impl ops::Deref for Object {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Default)]
pub struct ObjectId([u8; 20]);
impl ObjectId {
    pub fn new(oid: [u8; 20]) -> Self {
        Self(oid)
    }
}
impl ops::Deref for ObjectId {
    type Target = [u8; 20];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
