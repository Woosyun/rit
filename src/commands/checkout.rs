use crate::prelude::*;
use std::{
    fs,
    path::{PathBuf, Path},
};

pub struct Checkout {
    ws: Workspace,
    repo: Repository,
    curr_rev: Rev,
    target: Result<CheckoutTarget>,
}

enum CheckoutTarget {
    Oid(Oid),
    Branch(String),
}

impl Checkout {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        //check workspace is clean
        let prev_rev = repo.into_rev()?;
        let curr_rev = ws.into_rev()?;
        let rev_diff = prev_rev.diff(&curr_rev)?;
        if !rev_diff.is_clean() {
            return Err(Error::Workspace("Cannot checkout when workspace is not clean".to_string()));
        }

        let re = Self {
            ws,
            repo,
            curr_rev,
            target: Err(Error::Commands("checkout target is not set".into())),
        };
        Ok(re)
    }
    pub fn set_target_to_oid(&mut self, oid: Oid) {
        self.target = Ok(CheckoutTarget::Oid(oid));
    }
    pub fn set_target_to_branch(&mut self, branch: String) {
        self.target = Ok(CheckoutTarget::Branch(branch));
    }

    fn upsert_entry(&self, target_rev: &Rev, index: &Path) -> Result<()> {
        let entry = target_rev.get(index).unwrap();
        let mtime = entry.mtime();
        let oid = entry.oid()?;
        let blob: Blob = self.repo.db.retrieve(oid)?;
        let path = self.ws.workdir().join(index);
        fs::write(&path, blob)
            .map_err(|e| Error::Commands(e.to_string()))?;
        set_file_mtime(&path, mtime)
            .map_err(|e| Error::Commands(e.to_string()))
    }

    pub fn execute(&self) -> crate::Result<()> {
        let target_oid = match self.target.as_ref().map_err(Clone::clone)? {
            CheckoutTarget::Oid(oid) => oid.clone(),
            CheckoutTarget::Branch(branch) => {
                self.repo.refs.get(branch)?
                    .leaf().clone()
            }
        };
        let target_rev = Revision::build(self.repo.clone(), &target_oid)?
            .into_rev()?;

        let rev_diff = self.curr_rev.diff(&target_rev)?;

        for a in rev_diff.added.iter() {
            self.upsert_entry(&target_rev, a)?;
        }
        for m in rev_diff.modified.iter() {
            self.upsert_entry(&target_rev, m)?;
        }
        for r in rev_diff.removed.iter() {
            let path = self.ws.workdir().join(r);
            fs::remove_file(&path)
                .map_err(|e| Error::Commands(e.to_string()))?;
        }

        //clear empty directories

        match &self.target.as_ref().map_err(Clone::clone)? {
            CheckoutTarget::Oid(oid) => self.repo.local_head.set_to_oid(oid),
            CheckoutTarget::Branch(branch) => self.repo.local_head.set_to_branch(branch),
        }?;

        Ok(())
    }
}
