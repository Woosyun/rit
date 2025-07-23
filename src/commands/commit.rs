use crate::prelude::*;
use std::{
    fs,
    path::{PathBuf, Path},
    collections::VecDeque,
};

pub struct Commit {
    ws: Workspace,
    repo: Repository,
    parents: Vec<Oid>,
    branch: String,
    message: Result<String>,
}
impl Commit {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        //find oid of head and branch
        let (branch, parents) = if let Some(branch_name) = Self::read_head(&repo)? {
            let oid = repo.refs.get(&branch_name)?
                .leaf()
                .clone();

            (branch_name, Vec::from([oid]))
        } else {
            ("main".to_string(), Vec::new())
        };

        let cmd = Self {
            ws,
            repo,
            parents,
            branch,
            message: Err(Error::Commands("Commit not set".into())),
        };

        Ok(cmd)
    }

    /// - read local head and get branch.
    /// - if head is Oid, return error
    fn read_head(repo: &Repository) -> Result<Option<String>> {
        match repo.local_head.get()? {
            Head::None => Ok(None),
            Head::Branch(branch) => Ok(Some(branch)),
            _ => Err(Error::Commands("Cannot commit on non-branch revision".into())),
        }
    }
    pub fn add_parent(&mut self, parent: Oid) {
        self.parents.push(parent);
    }
    pub fn set_message(&mut self, message: String) {
        self.message = Ok(message);
    }
    
    fn store_blob_and_upsert_entry(&self, prev_rev: &mut Rev, curr_rev: &mut Rev, index: &Path) -> Result<()> {
        let path = self.ws.workdir().join(index);
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let blob = Blob::new(content);
        let oid = self.repo.db.store(&blob)?;

        let file = curr_rev.get_mut(index).unwrap();
        file.set_oid(oid);

        // Since oid of entries in curr_rev are None,
        // prev_rev should be used and updated
        let entry = repository::Entry::build(&**file)?;
        prev_rev.insert(index.to_path_buf(), Box::new(entry));
        Ok(())
    }
    fn store_tree_and_update_oid(&self, tree: &mut workspace::Tree) -> Result<()> {
        let repo_tree = tree
            .entries()
            .iter()
            .map(|(_, tree_entry)| {
                match tree_entry {
                    workspace::Entry::Tree(tree) => repository::Entry::build(tree),
                    workspace::Entry::Entry(entry) => repository::Entry::build(&**entry),
                }
            })
            .collect::<Result<Vec<_>>>()?;
        let oid = self.repo.db.store(&repo_tree)?;
        tree.set_oid(oid);

        Ok(())
    }
    
    pub fn execute(&self) -> crate::Result<()> {
        let mut prev_rev = self.repo.into_rev()?;
        let mut curr_rev = self.ws.into_rev()?;

        let rev_diff = prev_rev.diff(&curr_rev)?;
        if rev_diff.is_clean() {
            return Err(crate::Error::Workspace("Workspace is clean. Nothing to commit".into()));
        }

        // 2. store Blobs and update oid for entry
        for index in rev_diff.added.iter() {
            self.store_blob_and_upsert_entry(&mut prev_rev, &mut curr_rev, index)?;
        }
        for index in rev_diff.modified.iter() {
            self.store_blob_and_upsert_entry(&mut prev_rev, &mut curr_rev, index)?;
        }
        for index in rev_diff.removed.iter() {
            prev_rev.remove(index).unwrap();
        }

        // 3. store tree and update oid for entry
        let mut ws_tree = workspace::Tree::new(".".into());
        for (index, entry) in prev_rev {
            //let mut ancestors = self.ws.get_ancestors(&index)?;
            let mut components = index
                .components()
                .filter_map(|c| {
                    match c {
                        std::path::Component::Normal(name) => Some(name),
                        _ => None,
                    }
                })
                .map(|oss| oss.to_str().unwrap().to_string())
                .collect::<VecDeque<String>>();
            ws_tree.add_entry(&mut components, entry);
        }
        
        ws_tree.traverse_mut(|tree| self.store_tree_and_update_oid(tree))?;

        // 4. store commit and update head
        let root = ws_tree.oid()?.clone();
        let commit = repository::Commit::new(self.parents.clone(), root, self.message.clone()?);
        let new_head = self.repo.db.store(&commit)?;

        self.repo.refs.set(&self.branch, &new_head)?;

        Ok(())
    }
}
