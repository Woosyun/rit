mod workspace;
use workspace::*;


use rit::prelude::*;

pub trait GhostDiff
where
    T: Sized,
{
    fn ghost_diff(&self, another: T) -> Result<GhostDiffResult>;
}

pub enum GhostDiffResult {
    Same,
    Different(String),
    Unknown,
}
