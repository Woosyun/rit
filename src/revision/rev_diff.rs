use crate::prelude::*;

// todo: what if workspace/repository/revision can handle rev_diff?
pub trait HandleRevDiff {
    fn apply_rev_diff(&mut self, from: &impl HandleRevDiff, rev_diff: &RevDiff) -> Result<()>;
}
