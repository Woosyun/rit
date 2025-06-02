use crate::{
    prelude::*,
    repository::Commit,
};
use std::{
    path::PathBuf,
};

pub struct Log {
    repo: Repository,
}
impl Log {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            repo,
        })
    }

    pub fn read_all_branches(&self) -> Result<Vec<Node>> {
        let mut nodes = Vec::new();

        for branch in self.repo.refs.list_branches()? {
            if self.repo.refs.contains(&branch) {
                let oid = self.repo.refs.get(&branch)?;
                nodes.push(self.node(&oid)?);
            }
        }

        Ok(nodes)
    }

    pub fn node(&self, oid: &Oid) -> Result<Node> {
        let commit: Commit = self.repo.db.retrieve(oid)?;
        let node = Node {
            repo: &self.repo,
            oid: oid.clone(),
            commit
        };

        Ok(node)
    }
}

pub struct Node<'a> {
    repo: &'a Repository,
    oid: Oid,
    commit: Commit,
}
impl<'a> Node<'a> {
    pub fn commit(&self) -> &Commit {
        &self.commit
    }
    pub fn oid(&self) -> &Oid {
        &self.oid
    }
}
impl<'a> Iterator for Node<'a> {
    type Item = Node<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(oid) = self.commit.parent() {
            let parent: Commit = self.repo.db.retrieve(&oid)
                .unwrap_or_else(|_| None)?;
            let parent_node= Node {
                repo: self.repo,
                oid: oid.to_owned(),
                commit: parent,
            };
            Some(parent_node)
        } else {
            None
        }
    }
}
