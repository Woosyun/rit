#![allow(unused)]

mod test_utils;

mod worker;
pub use worker::Worker;

mod driver;
pub use driver::Driver;

use rand::prelude::*;
use std::{
    path::{PathBuf, Path},
    collections::HashSet,
    io,
    fs,
};
use rit::{
    self,
    prelude::*,
};
use tempdir::TempDir;

#[derive(Debug)]
pub struct Client {
    pub tempdir: TempDir,
    pub added: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
    pub removed: HashSet<PathBuf>,
}

impl Client {
    pub fn build(test_name: &str) -> Result<Self> {
        let tempdir = TempDir::new(test_name)
            .map_err(|e| Error::Workspace(e.to_string()))?;

        Ok(Self {
            tempdir,
            added: HashSet::new(),
            modified: HashSet::new(),
            removed: HashSet::new(),
        })
    }
    fn init(&self) -> Result<()> {
        let cmd = commands::Init::build(self.workspace()?.workdir().to_path_buf())?;
        cmd.execute()
    }
    pub fn try_init(&self) -> Result<()> {
        self.init()?;

        let repo = self.workspace()?.workdir().join(Repository::name());
        assert!(repo.exists());

        let db = repo.join(Database::name());
        assert!(db.exists());

        let mut refs = repo.join(Refs::name());
        refs.push(Refs::local());
        assert!(refs.exists());

        let local_head = repo.join(LocalHead::name());
        assert!(local_head.exists());

        self.check_workspace_status()
    }

    pub fn try_work(&mut self) -> rit::Result<()> {
        self.work_random()?;
        self.check_workspace_status()
    }

    fn commit(&self) -> rit::Result<()> {
        let mut cmd = rit::commands::Commit::build(self.workdir().to_path_buf())?;
        let message = format!("commit-{}", rand::rng().random::<u32>());
        cmd.set_message(message);
        cmd.execute()
    }
    pub fn try_commit(&mut self) -> rit::Result<()> {
        self.commit()?;

        //check deletion worked
        for file in self.removed.iter() {
            let path = self.workdir().join(file);
            if path.exists() {
                let f = format!("{:?} not removed", file);
                return Err(rit::Error::Workspace(f));
            }
        }

        //check addition/modification worked
        let repo = self.repository()?;
        let compare_blobs = |file: &Path| -> rit::Result<()> {
            let path = self.workdir().join(file);
            let content = fs::read_to_string(&path)
                .map_err(|e| Error::Workspace(e.to_string()))?;
            let blob_ws = Blob::new(content);
            let content = serde_json::to_string(&blob_ws)
                .map_err(|e| Error::Workspace(e.to_string()))?;
            let oid = Oid::build(&content);
            assert!(repo.db.retrieve::<Blob>(&oid).is_ok());

            Ok(())
        };
        for file in self.added.iter() {
            compare_blobs(file)?;
        }
        for file in self.modified.iter() {
            compare_blobs(file)?;
        }

        self.added.clear();
        self.modified.clear();
        self.removed.clear();

        self.check_workspace_status()
    }

    //check integrity of workspace?
    fn check_workspace_status(&self) -> Result<()> {
        let rev_diff = self.repository()?
            .into_rev()?
            .diff(&self.workspace()?.into_rev()?)?;

        assert_eq!(self.added, rev_diff.added, "comparing added files for one that was recorded and one returned by status command");
        assert_eq!(self.modified, rev_diff.modified, "comparing modified files for one that was recorded and one returned by status command");
        assert_eq!(self.removed, rev_diff.removed, "comparing removed files for one that was recorded and one returned by status command");

        Ok(())
    }

    pub fn try_checkout(&self, branch: &str) -> Result<()> {
        let repo = self.repository()?;
        let original_branch = repo.local_head.get()?
            .branch()?.to_string();
        let original_rev = repo.into_rev()?;

        let target_oid = repo.refs.get(branch)?;
        let target_rev = Revision::build(repo.clone(), &target_oid)?
            .into_rev()?;

        let checkout = commands::Checkout::build(self.workdir().to_path_buf())?;
        checkout.execute(branch)?;
        assert_eq!(repo.local_head.get()?.branch()?, branch);

        //check whether checkout conducted
        // but if original branch and target branch are same,
        // then even if checkout didn't work,
        // cannot check properly
        let repo = self.repository()?;
        let current_rev = repo.into_rev()?;
        assert!(current_rev.diff(&target_rev)?.is_clean());

        let original_oid = repo.refs.get(&original_branch)?;
        let expected_original_rev = Revision::build(repo.clone(), &original_oid)?
            .into_rev()?;
        assert!(original_rev.diff(&expected_original_rev)?.is_clean());


        Ok(())
    }

    pub fn try_branch_create(&self, new_branch: &str) -> Result<()> {
        let branch = commands::Branch::build(self.workdir().to_path_buf())?;
        branch.create(new_branch)?;

        Ok(())
    }

    pub fn try_merge_branch(&self, to: &str) -> Result<()> {
        let mut merge = commands::Merge::build(self.workdir().to_path_buf())?;
        merge.set_target_branch(to.to_string());
        merge.execute()?;

        Ok(())
    }
}

impl Driver for Client {
    fn workdir(&self) -> &Path {
        self.tempdir.path()
    }
    fn workspace(&self) -> Result<Workspace> {
        Workspace::build(self.workdir().to_path_buf())
    }
    fn repository(&self) -> Result<Repository> {
        Repository::build(&self.workspace()?)
    }
}

impl Worker for Client {
    fn added(&mut self) -> &mut HashSet<PathBuf> {
        &mut self.added
    }
    fn modified(&mut self) -> &mut HashSet<PathBuf> {
        &mut self.modified
    }
    fn removed(&mut self) -> &mut HashSet<PathBuf> {
        &mut self.removed
    }
}
