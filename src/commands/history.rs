use crate::prelude::*;
use std::{
    collections::HashMap,
    path::PathBuf,
};
use serde::{Serialize, Deserialize};

pub struct History {
    repo: Repository,
}
impl History {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            repo,
        })
    }

    fn rec_read(&self, hg: &mut HistoryGraph, oid: &Oid, count: usize) -> Result<()> {
        if !hg.commits().get(oid).is_some() || count == 0 {
            return Ok(());
        } 

        let commit: repository::Commit = self.repo.db.retrieve(oid)?;
        hg.insert_commit(oid.clone(), commit.clone());
        for parent in commit.parents() {
            self.rec_read(hg, parent, count-1)?;
        }
        Ok(())
    }

    //todo: make generalized read method

    pub fn read_full(&self) -> Result<HistoryGraph> {
        let mut hg = HistoryGraph::new();

        for branch_name in self.repo.refs.list_branches()? {
            //set nodes
            let branch = self.repo.refs.get(&branch_name)?;
            self.rec_read(&mut hg, &branch.leaf(), 10)?;
            
            //set branch->leaf node
            hg.insert_branch(branch_name, branch);
        }

        Ok(hg)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryGraph {
    commits: HashMap<Oid, repository::Commit>,

    //each branch points to root, leaf revision,
    branches: HashMap<String, Branch>,
}
impl HistoryGraph {
    pub fn new() -> Self {
        Self {
            commits: HashMap::new(),
            branches: HashMap::new(),
        }
    }
    
    fn insert_commit(&mut self, oid: Oid, commit: repository::Commit) -> Option<repository::Commit> {
        self.commits.insert(oid, commit)
    }
    pub fn commits(&self) -> &HashMap<Oid, repository::Commit> {
        &self.commits
    }

    fn insert_branch(&mut self, name: String, branch: Branch) -> Option<Branch> {
        self.branches.insert(name, branch)
    }
    pub fn branches(&self) -> &HashMap<String, Branch> {
        &self.branches
    }
}
