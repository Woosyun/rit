// tothink:
// Is this neccessary?
// Why not just let commands
// handle enum of local/remote branch, oid?

use crate::prelude::*;
use std::fmt;

pub const SYM_REF_PREFIX: &str = "ref: ";

pub enum Reference {
    Oid(ObjectId),
    Sym(RelativePath),
    // LocalBranch(String)
}
impl Reference {
    pub fn is_oid(&self) -> bool {
        matches!(self, Self::Oid(_))
    }
    pub fn is_symref(&self) -> bool {
        !self.is_oid()
    }

    pub fn oid(&self) -> Option<&ObjectId> {
        match self {
            Reference::Oid(oid) => Some(oid),
            _ => None,
        }
    }
    pub fn relative_path(&self) -> Option<&RelativePath> {
        match self {
            Reference::Sym(relative_path) => Some(relative_path),
            _ => None,
        }
    }
}
impl From<ObjectId> for Reference {
    fn from(oid: ObjectId) -> Self {
        Self::Oid(oid)
    }
}
// impl From<RelativePath> for Reference {
//     fn from(relative_path: RelativePath) -> Self {
//         Self::Sym(relative_path)
//     }
// }

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::Oid(oid) => write!(f, "{}", oid),
            Reference::Sym(sym_ref) => write!(f, "{}", sym_ref.to_string_lossy()),
        }
    }
}
