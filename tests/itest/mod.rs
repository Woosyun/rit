#![allow(unused)]

mod fs;
mod utils;

use rand::prelude::*;
use std::{
    path::{PathBuf, Path},
    collections::HashSet,
    io,
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
    pub fn build(test_name: &str) -> io::Result<Self> {
        let tempdir = TempDir::new(test_name)?;

        Ok(Self {
            tempdir,
            added: HashSet::new(),
            modified: HashSet::new(),
            removed: HashSet::new(),
        })
    }
    pub fn workdir(&self) -> &Path {
        self.tempdir.path()
    }
    pub fn workspace(&self) -> rit::Result<Workspace> {
        Workspace::build(self.workdir().to_path_buf())
    }
    pub fn repository(&self) -> rit::Result<Repository> {
        Repository::build(&self.workspace()?)
    }

    fn init(&self) -> Result<()> {
        let cmd = rit::commands::Init::build(self.workspace()?.path().to_path_buf())?;
        cmd.execute()
    }
    pub fn try_init(&self) -> Result<()> {
        self.init()?;

        let repo = self.workspace()?.path().join(Repository::name());
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
        //why?
        utils::sleep_1_sec();

        let ws = self.workspace()?;
        let curr_rev = ws.into_rev()
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e.to_string()))?;

        let mut files = curr_rev.0.keys().cloned().collect::<Vec<_>>();
        let mut rng = rand::rng();
        files.shuffle(&mut rng);

        let number_of_files = files.len();

        //remove
        let number_of_deletion = number_of_files/3;
        for file in files.iter().take(number_of_deletion) {
            self.removed.insert(file.to_path_buf());

            let path = self.workdir().join(file);
            if path.exists() {
                fs::remove_file(&path)?;
            }
        }

        //modify
        let remaining_files = files
            .iter()
            .skip(number_of_deletion)
            .cloned()
            .collect::<Vec<_>>();
        let number_of_modification = number_of_files/3;
        for file in remaining_files.iter().take(number_of_modification) {
            self.modified.insert(file.to_path_buf());

            let path = self.workdir().join(&file);
            fs::appendln(&path, "\n//modified for integration testing")?;
        }

        //add
        let number_of_creation = if number_of_files < 10 {
            10
        } else {
            number_of_files/3
        };
        for i in 0..number_of_creation {
            let new_file = format!("new_file_{}_{}.txt", i, rng.random::<u32>());
            self.added.insert(Path::new(&new_file).to_path_buf());

            let path = self.workdir().join(new_file);
            if !path.exists() {
                fs::write(&path, "newly created for integration testing")?;
            }
        }

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
            let blob_ws = Blob::new(fs::read_to_string(&path)?);
            let json = decode(&blob_ws).unwrap();
            let oid = Oid::build(&json);
            let blob_db: Blob = repo.db.retrieve(&oid)?;

            assert_eq!(blob_ws, blob_db);

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
    pub fn check_workspace_status(&self) -> Result<()> {
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

    //todo: 
    // to test merge, 
    // lower file handling ability might be needed
}
