use crate::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct RevDiff {
    pub added: HashSet<Index>,
    pub removed: HashSet<Index>,
    pub modified: HashSet<Index>,
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
