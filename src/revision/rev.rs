use crate::prelude::*;
use std::{
    collections::HashMap,
    path::PathBuf,
};
//use serde::{Serialize, Deserialize};

pub trait IntoRev {
    fn into_rev(&self) -> Result<Rev>;
}

type InnerRev = HashMap<PathBuf, Box<dyn Stat>>;
pub struct Rev(InnerRev);
impl Rev {
    pub fn new(rev: InnerRev) -> Self {
        Self(rev)
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

impl std::ops::Deref for Rev {
    type Target = InnerRev;

    fn deref(&self) -> &InnerRev {
        &self.0
    }
}
impl std::ops::DerefMut for Rev {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl IntoIterator for Rev {
    type Item = (PathBuf, Box<dyn Stat>);
    type IntoIter = std::collections::hash_map::IntoIter<PathBuf, Box<dyn Stat>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
