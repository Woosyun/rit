pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod revision;
pub use revision::*;

pub type Object = Vec<u8>;
pub type ObjectId = [u8; 20];
