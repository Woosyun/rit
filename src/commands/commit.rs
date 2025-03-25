use crate::{
    repository::{Repository, database},
    workspace::{self, Workspace},
};
use std::path::PathBuf;

pub struct Commit {
    repo: Repository,
    ws: Workspace
}
impl Commit {
    pub fn build(cwd: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(cwd)?;
        let repo = Repository::build(&ws)?;
        let commit = Self {
            repo,
            ws
        };

        Ok(commit)
    }

    pub fn execute(&self, msg: String) -> crate::Result<&'static str> {
        let db = self.repo.get_database()?;


        let mut tree = workspace::Tree::new();
        for file in self.ws.list_files(None)? {
            let blob = self.ws.read_to_blob(&file)?;
            let stat = self.ws.read_stat(&file)?;
            let oid = db.store(&blob)?;
            let name = file.file_name().expect("path terminated with ..")
                .to_str().expect("file name is not valid unicode")
                .to_string();
            let entry = database::Entry::from_blob(stat, oid, name);
            let ancestors = self.ws.ancestors(&file)?;
            tree.add_entry(ancestors, entry);
        }

        //store tree recursively
        let handler = |tree: &mut workspace::Tree| -> crate::Result<()> {
            //change entries to Entry::Entry(database::Entry)
            let entries = tree
                .entries
                .iter()
                .map(|(name, entry)| {
                    match entry {
                        workspace::Entry::Tree(tree) => {
                            let oid = tree.oid.as_ref().expect("tree didn't get oid");
                            let entry = database::Entry::from_tree(oid.clone(), name.clone());
                            entry
                        },
                        workspace::Entry::Entry(entry) => entry.clone()
                    }
                })
                .collect::<Vec<_>>();
            let db_tree = database::Tree::new(entries);
            let oid = db.store(&db_tree)?;
            tree.oid = Some(oid);

            Ok(())
        };
        tree.traverse_mut(handler)?;

        let root_tree_oid = tree.oid.expect("commit didn't work well");
        //get previous head
        let head = self.repo.get_head();
        let refs = self.repo.get_refs();
        let previous_commit_oid = if let Some(branch) = head.read()? {
            Some(refs.read(&branch)?)
        } else {
            None
        };

        let commit = database::Commit::new(previous_commit_oid, root_tree_oid, msg);
        let new_head_oid = db.store(&commit)?;
        if let Some(branch) = head.read()? {
            refs.write(&branch, &new_head_oid)?;
            head.write(&branch)?;
        } else {
            refs.init(&new_head_oid)?;
        }

        Ok("commit worked")
    }
}
