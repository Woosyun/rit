use crate::prelude::*;
use std::{
    path::PathBuf,
};

pub struct Branch {
    repo: Repository,
}
impl Branch {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        Ok(Self {
            repo
        })
    }

    // create new branch and copy-paste head
    pub fn create(&self, new_branch: &str) -> Result<()> {
        if self.repo.refs.contains(new_branch) {
            return Err(Error::Repository("branch is already exists.".into()));
        }

        let head = self.repo.local_head.get()?;
        let oid = if head.is_branch() {
            &self.repo.refs.get(head.branch()?)?
        } else {
            head.oid()?
        };
        self.repo.refs.set(new_branch, oid)?;

        Ok(())
    }
}
