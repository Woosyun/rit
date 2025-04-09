use crate::{
    workspace::Stat,
    //repository::Blob,
};
use std::{
    collections::{HashSet, HashMap},
    path::{PathBuf, Path},
};
//use serde::{Serialize, Deserialize};

pub trait IntoRev {
    fn into_rev(&self) -> crate::Result<Rev>;
}

pub struct Rev(pub HashMap<PathBuf, Box<dyn Stat>>);
impl Rev {
    pub fn new(rev: HashMap<PathBuf, Box<dyn Stat>>) -> Self {
        Self(rev)
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
        for (path, to_entry) in to.0.iter() {
            if let Some(from_entry) = self.0.get(path) {
                if from_entry.mtime() != to_entry.mtime() {
                    rev_diff.modified.insert(path.to_path_buf());
                }
            } else {
                rev_diff.added.insert(path.to_path_buf());
            }
        }

        Ok(rev_diff)
    }
}

pub struct RevDiff {
    pub added: HashSet<PathBuf>,
    pub removed: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
}
impl RevDiff {
    fn new() -> Self {
        Self {
            added: HashSet::new(),
            removed: HashSet::new(),
            modified: HashSet::new(),
        }
    }
    pub fn is_clean(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.modified.is_empty()
    }
}
