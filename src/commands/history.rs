use crate::prelude::*;
use std::{
    collections::{HashMap, HashSet},
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

    fn read_branches(&self) -> Result<Vec<Oid>> {
        let mut leaves = Vec::new();

        for branch in self.repo.refs.list_branches()? {
            if self.repo.refs.contains(&branch) {
                if let Ok(oid) = self.repo.refs.get(&branch) {
                    leaves.push(oid);
                }
            }
        }

        Ok(leaves)
    }

    fn traverse(&self, hg: &mut HistoryGraph, child: &Oid, count: u32) -> Result<()> {
        if count == 0 {
            hg.add_root(child.clone());
            return Ok(());
        }

        let commit: repository::Commit = self.repo.db.retrieve(child)?;
        for parent in commit.parents() {
            hg.add_edge(parent.clone(), child.clone());
            self.traverse(hg, parent, count-1)?;
        }
        Ok(())
    }

    pub fn read_full(&self) -> Result<HistoryGraph> {
        let mut hg = HistoryGraph::new();
        for leaf in self.read_branches()? {
            self.traverse(&mut hg, &leaf, 1000)?;
        }
        Ok(hg)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryGraph {
    children: HashMap<Oid, HashSet<Oid>>,
    roots: HashSet<Oid>,
}
impl HistoryGraph {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            roots: HashSet::new(),
        }
    }
    
    fn add_edge(&mut self, from: Oid, to: Oid) {
        self.children
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
    }
    fn add_root(&mut self, root: Oid) {
        self.roots.insert(root);
    }

    pub fn children(&self) -> &HashMap<Oid, HashSet<Oid>> {
        &self.children
    }
    pub fn roots(&self) -> &HashSet<Oid> {
        &self.roots
    }
}
