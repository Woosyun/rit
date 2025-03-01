use sha2::{Sha256, Digest};
use std::{
    hash::Hash,
    io
};
use crate::prelude::*;

// todo: objectify can fail. Maybe change objectify trait?
pub trait CalculateHash: Hash + Objectify {
    fn calculate_hash(&self) -> io::Result<Hash256> {
        let mut hasher = Sha256::new();
        hasher.update(self.objectify());

        let hash = hasher.finalize()
            .into_iter()
            .collect::<Vec<_>>();

        Hash256::new(hash)
    }
}
pub trait Objectify {
    fn objectify(&self) -> String;
}
