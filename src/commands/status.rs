use std::path::PathBuf;
use crate::prelude::*;

pub struct Status {
    ws: Workspace,
    repo: Repository,
}
impl Status {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            ws,
            repo,
        })
    }
    
    pub fn execute(&self) -> crate::Result<RevDiff> {
        let prev_rev = self.repo.into_rev()?;
        let curr_rev = self.ws.into_rev()?;

        let rev_diff = prev_rev.diff(&curr_rev)?;
        Ok(rev_diff)
    }

    // read working directory and report status about its repository

    /*
    pub fn scan(wd: PathBuf) -> Result<RepositoryStatus> {
        let mut path = wd.clone();
        if !path.exists() {
            return Ok(RepositoryStatus::InvalidPath);
        }
        path.push(Repository::name());
        if !path.exists() {
            return Ok(RepositoryStatus::NotFound);
        }

        let ws = Workspace::build(wd)?;
        let repo = Repository::build(&ws)?;
        let head = repo.local_head.get()?;
        if !head.is_branch() {
            return Ok(RepositoryStatus::NotBranch);
        }

        Ok(RepositoryStatus::Normal)
    }
    */
}

#[derive(Debug)]
pub enum RepositoryStatus {
    InvalidPath,
    NotFound,
    NotBranch,
    Normal
}
impl RepositoryStatus {
    pub fn is_repository_initialized(&self) -> bool {
        use RepositoryStatus::*;
        match self {
            InvalidPath => false,
            NotFound => false,
            _ => true,
        }
    }
    
    pub fn is_path_valid(&self) -> bool {
        use RepositoryStatus::*;
        match self {
            InvalidPath => false,
            _ => true,
        }
    }
}
