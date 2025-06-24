use crate::prelude::*;
use std::{
    collections::HashSet,
    path::PathBuf,
};

#[derive(PartialEq, Debug)]
pub struct RevDiff {
    pub added: HashSet<PathBuf>,
    pub removed: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
}
impl RevDiff {
    pub fn new() -> Self {
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

// todo: what if workspace/repository/revision can handle rev_diff?
pub trait HandleRevDiff {
    fn apply_rev_diff(&mut self, from: &impl HandleRevDiff, rev_diff: &RevDiff) -> Result<()>;
}
