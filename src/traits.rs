use sha2::{Sha256, Digest};
use std::hash::Hash;

pub trait CalculateHash: Hash + Objectify {
    fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.objectify());
        let result = hasher.finalize();

        result
            .into_iter()
            //.map(|c| c.to_string())
            .collect::<Vec<_>>()
    }
}
pub trait Objectify {
    fn objectify(&self) -> String;
}
