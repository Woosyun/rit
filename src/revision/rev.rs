use crate::prelude::*;
use std::{
    collections::HashMap,
    path::{PathBuf, Path},
};
//use serde::{Serialize, Deserialize};

pub trait IntoRev {
    fn into_rev(&self) -> Result<Rev>;
}

pub struct Rev(HashMap<PathBuf, Box<dyn Stat>>);
impl Rev {
    pub fn new(rev: HashMap<PathBuf, Box<dyn Stat>>) -> Self {
        Self(rev)
    }

    pub fn get(&self, idx: &Path) -> Option<&Box<dyn Stat>> {
        self.0.get(idx)
    }
    pub fn get_mut(&mut self, idx: &Path) -> Option<&mut Box<dyn Stat>> {
        self.0.get_mut(idx)
    }

    pub fn diff(&self, to: &Rev) -> crate::Result<RevDiff> {
        let mut rev_diff = RevDiff::new();
        for (path, _) in self.0.iter() {
            if to.0.get(path).is_none() {
                rev_diff.removed.insert(path.to_path_buf());
            }
        }
        for (index, to_entry) in to.0.iter() {
            if let Some(from_entry) = self.0.get(index) {
                if from_entry.mtime() != to_entry.mtime() {
                    rev_diff.modified.insert(index.to_owned());
                }
            } else {
                rev_diff.added.insert(index.to_owned());
            }
        }

        Ok(rev_diff)
    }
}
impl Iterator for Rev {
    fn next(&mut self) ->  
}
