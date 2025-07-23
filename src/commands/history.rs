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

    fn rec_read(&self, hg: &mut HistoryGraph, oid: &Oid, count: usize) -> Result<()> {
        if !hg.insert_node(oid.clone()) {
            return Ok(());
        } else if count == 0 {
            return Ok(());
        }

        let commit: repository::Commit = self.repo.db.retrieve(oid)?;
        for parent in commit.parents() {
            self.rec_read(hg, parent, count-1)?;
            hg.insert_parent(oid.clone(), parent.clone());
        }
        Ok(())
    }

    pub fn read_full(&self) -> Result<HistoryGraph> {
        let mut hg = HistoryGraph::new();

        for branch_name in self.repo.refs.list_branches()? {
            //set nodes
            let branch = self.repo.refs.get(&branch_name)?;
            let leaf = branch.leaf();
            self.rec_read(&mut hg, &leaf, 100)?;
            
            //set branch->leaf node
            hg.insert_branch(branch_name, leaf.clone());
        }

        Ok(hg)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryGraph {
    //roots: HashSet<Oid>,

    //known Oids pointing each revision
    nodes: HashSet<Oid>,

    //parent -> children
    parents: HashMap<Oid, HashSet<Oid>>,

    //each branch points to leaf revision,
    //todo: branch should be consists of leaf(end) and root(start) oids
    //maybe create graph first and pick one of the children's start branches
    //and move upward one that has higher hierarchy.
    branches: HashMap<String, Oid>,
}
impl HistoryGraph {
    pub fn new() -> Self {
        Self {
            //roots: HashSet::new(),
            nodes: HashSet::new(),
            parents: HashMap::new(),
            branches: HashMap::new(),
        }
    }
    
    fn insert_node(&mut self, node: Oid) -> bool {
        self.nodes.insert(node)
    }
    pub fn nodes(&self) -> &HashSet<Oid> {
        &self.nodes
    }

    fn insert_branch(&mut self, branch: String, oid: Oid) -> Option<Oid> {
        self.branches.insert(branch, oid)
    }
    pub fn branches(&mut self) -> &HashMap<String, Oid> {
        &self.branches
    }

    fn insert_parent(&mut self, child: Oid, parent: Oid) {
        self.parents
            .entry(child)
            .or_default()
            .insert(parent);
    }
    pub fn parents(&mut self) -> &HashMap<Oid, HashSet<Oid>> {
        &self.parents
    }
}
