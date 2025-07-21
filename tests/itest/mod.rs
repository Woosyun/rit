pub mod worker;
pub use worker::*;

pub mod driver;
pub use driver::*;


use std::{
    path::{PathBuf, Path},
    collections::HashSet,
    fs,
    fmt::Write,
};
use rit::{
    prelude::*,
    commands::*,
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
    #[allow(unused)]
    pub fn print_status(&self) -> Result<()> {
        let mut output = String::new();

        let ws_rev = self.workspace()?
            .into_rev()?;
        writeln!(output, "\nall indices of workspace: ")
            .map_err(|e| Error::Workspace(e.to_string()))?;
        for (index, _) in ws_rev {
            writeln!(output, "{:?}", self.read_to_file(&index)?)
                .map_err(|e| Error::Workspace(e.to_string()))?;
        }

        writeln!(output, "added: {:?}", self.added)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        writeln!(output, "modified: {:?}", self.modified)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        writeln!(output, "removed: {:?}", self.removed)
            .map_err(|e| Error::Workspace(e.to_string()))?;

        
        println!("{output}");
        Ok(())
    }
    pub fn try_init(&self) -> Result<()> {
        println!("try init");
        let cmd = init::Init::build(self.workdir().to_path_buf())?;
        cmd.execute()?;

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
        println!("try to work");
        self.work_random()?;
        self.check_workspace_status()
    }

    pub fn try_commit(&mut self) -> rit::Result<()> {
        println!("try to commit");
        let mut cmd = commit::Commit::build(self.workdir().to_path_buf())?;
        cmd.set_message("commit message".to_string());
        cmd.execute()?;

        //check deletion worked
        for index in self.removed.iter() {
            let path = self.workdir().join(index);
            if path.exists() {
                let f = format!("{:?} was not removed", index);
                return Err(rit::Error::Workspace(f));
            }
        }

        //check addition/modification worked
        let repo = self.repository()?;
        let compare_blobs = |index: &Path| -> rit::Result<()> {
            let path = self.workdir().join(index);
            let content = fs::read_to_string(&path)
                .map_err(|e| Error::Workspace(e.to_string()))?;
            let blob_ws = Blob::new(content);
            let content = serde_json::to_string(&blob_ws)
                .map_err(|e| Error::Workspace(e.to_string()))?;
            let oid = Oid::build(&content);

            if repo.db.retrieve::<Blob>(&oid).is_err() {
                let f = format!("file {:?} is not in the database", index);
                Err(Error::Database(f))
            } else {
                Ok(())
            }
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
        println!("try to checkout to '{}'", branch);
        let repo = self.repository()?;
        let get_current_branch = || if let Head::Branch(branch) = repo.local_head.get()? {
            Ok(branch)
        } else {
            return Err(Error::Commands("tried to get branch on non-branch in try_checkout".into()));
        };
        let original_rev = repo.into_rev()?;

        let target_oid = repo.refs.get(branch)?;
        let target_rev = Revision::build(repo.clone(), &target_oid)?
            .into_rev()?;

        let mut cmd = checkout::Checkout::build(self.workdir().to_path_buf())?;
        cmd.set_target_to_branch(branch.to_string());
        cmd.execute()?;
        assert_eq!(get_current_branch()?, branch);

        //check whether checkout conducted
        // but if original branch and target branch are same,
        // then even if checkout didn't work,
        // cannot check properly
        //let repo = self.repository()?;
        let current_rev = repo.into_rev()?;
        assert!(current_rev.diff(&target_rev)?.is_clean());

        let original_oid = repo.refs.get(&get_current_branch()?)?;
        let expected_original_rev = Revision::build(repo.clone(), &original_oid)?
            .into_rev()?;
        assert!(original_rev.diff(&expected_original_rev)?.is_clean());


        Ok(())
    }

    pub fn try_branch_create(&self, new_branch: &str) -> Result<()> {
        println!("try to create branch '{}'", new_branch);
        let cmd = branch::Branch::build(self.workdir().to_path_buf())?;
        cmd.create(new_branch)?;

        Ok(())
    }

    pub fn try_merge_branch(&self, to: &str) -> Result<()> {
        println!("try to merge branch '{}'", to);
        let mut cmd = merge::Merge::build(self.workdir().to_path_buf())?;
        cmd.set_target_branch(to.to_string());
        cmd.execute()
    }
}

impl Driver for Client {
    fn workdir(&self) -> &Path {
        self.tempdir.path()
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
