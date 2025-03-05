use sha2::{Sha256, Digest};
use std::{
    hash::Hash,
    io,
    path::PathBuf,
};
use crate::prelude::*;

// todo: objectify can fail. Maybe change objectify trait?
pub trait CalculateHash: Objectify {
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

pub trait Repository {
    fn get_repository(&self) -> PathBuf;
}
pub trait Database: Repository {
    fn get_database(&self) -> PathBuf {
        let mut obj = self.get_repository();
        obj.push("objects");
        obj.to_path_buf()
    }
}
