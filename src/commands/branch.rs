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

    fn read_head(&self) -> Result<Oid> {
        match self.repo.local_head.get()? {
            Head::None => Err(Error::Commands("Cannot read head on non-branch".into())),
            Head::Oid(oid) => Ok(oid),
            Head::Branch(branch) => {
                self.repo.refs.get(&branch)
            }
        }
    }

    // create new branch and copy-paste head
    pub fn create(&self, new_branch: &str) -> Result<()> {
        if self.repo.refs.contains(new_branch) {
            return Err(Error::Repository("branch is already exists.".into()));
        }

        let oid = self.read_head()?;
        self.repo.refs.set(new_branch, &oid)?;

        Ok(())
    }
}
