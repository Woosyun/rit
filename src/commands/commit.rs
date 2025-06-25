use crate::prelude::*;
use std::{
    fs,
    path::{PathBuf, Path},
};

pub struct Commit {
    ws: Workspace,
    repo: Repository,
    parent: Vec<Oid>,
    branch: String,
    message: String,
}
impl Commit {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        // get current branch name
        let head = repo.local_head.get()?;
        let branch = if head.is_branch() {
            head.branch()?.to_string()
        } else {
            return Err(Error::Repository("cannot run commit on non-branch head".into()));
        };
        let mut parent = Vec::new();
        if repo.refs.contains(&branch) {
            parent.push(repo.refs.get(&branch)?);
        }

        let cmd = Self {
            ws,
            repo,
            parent,
            branch,
            message: "".to_string(),
        };

        Ok(cmd)
    }
    pub fn add_parent(&mut self, parent: Oid) {
        self.parent.push(parent);
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
    
    fn store_blob_upsert_entry(&self, prev_rev: &mut Rev, curr_rev: &mut Rev, index: &Path) -> Result<()> {
        let path = self.ws.workdir().join(&index);
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let blob = Blob::new(content);
        let oid = self.repo.db.store(&blob)?;

        let file = curr_rev.get_mut(index).unwrap();
        file.set_oid(oid);

        // since oid of entry in curr_rev is None if its content was not changed,
        // prev_rev should be used here.
        let entry = repository::Entry::build(file.as_ref())?;
        prev_rev.insert(index.to_path_buf(), Box::new(entry));
        Ok(())
    }
    fn store_tree_update_oid(&self, tree: &mut workspace::Tree) -> Result<()> {
        let repo_tree = tree
            .entries
            .iter()
            .map(|(_, tree_entry)| {
                match tree_entry {
                    workspace::Entry::Tree(tree) => repository::Entry::build(tree),
                    workspace::Entry::Entry(entry) => repository::Entry::build(entry.as_ref()),
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
            self.store_blob_upsert_entry(&mut prev_rev, &mut curr_rev, index)?;
        }
        for index in rev_diff.modified.iter() {
            self.store_blob_upsert_entry(&mut prev_rev, &mut curr_rev, index)?;
        }
        for index in rev_diff.removed.iter() {
            prev_rev.remove(index).unwrap();
        }

        // 3. store tree and update oid for entry
        let mut ws_tree = workspace::Tree::new("".into());
        for (index, entry) in prev_rev {
            let mut ancestors = self.ws.get_ancestors(&index)?;
            ws_tree.add_entry(&mut ancestors, entry);
        }
        
        ws_tree.traverse_mut(|tree: &mut workspace::Tree| self.store_tree_update_oid(tree))?;

        // 4. store commit and update head
        let root = ws_tree.oid()?.clone();
        let commit = repository::Commit::new(self.parent.clone(), root, self.message.clone());
        let new_head = self.repo.db.store(&commit)?;

        self.repo.refs.set(&self.branch, &new_head)?;

        Ok(())
    }
}
