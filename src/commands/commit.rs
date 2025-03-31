use crate::{repository, workspace};
use std::path::PathBuf;

pub struct Commit {
    ws: workspace::Workspace,
    db: repository::Database,
    head: repository::Head,
    refs: repository::Refs,
    ignore: repository::Ignore
}
impl Commit {
    pub fn build(cwd: PathBuf) -> crate::Result<Self> {
        let ws = workspace::Workspace::build(cwd)?;
        let repo = repository::Repository::build(&ws)?;
        let db = repo.get_database()?;
        let head = repo.get_head();
        let refs = repo.get_refs();
        let ignore = repo.get_ignore()?;
        let commit = Self {
            ws,
            db,
            head,
            refs,
            ignore
        };

        Ok(commit)
    }

    pub fn execute(&self, msg: String) -> crate::Result<()> {
        let mut tree = workspace::Tree::new();
        for file in self.ws.list_files(None)? {
            let blob = self.ws.read_to_blob(&file)?;
            let oid = self.db.store(&blob)?;

            let stat = self.ws.read_stat(&file)?;
            let file_name = self.ws.get_file_name(&file)?;

            if !self.ignore.is_ignored(&file_name) {
                let entry = repository::Entry::from_blob(stat, oid, file_name);
                let mut ancestors = self.ws.get_ancestors(&file)?;
                tree.add_entry(&mut ancestors, entry);
            }
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
                            let entry = repository::Entry::from_tree(oid.clone(), name.clone());
                            entry
                        },
                        workspace::Entry::Entry(entry) => entry.clone()
                    }
                })
                .collect::<Vec<_>>();
            let db_tree = repository::Tree::new(entries);
            let oid = self.db.store(&db_tree)?;
            tree.oid = Some(oid);

            Ok(())
        };
        tree.traverse_mut(handler)?;

        let root_tree_oid = tree.oid.unwrap();
        //get previous head
        let previous_commit_oid = if let Some(branch) = self.head.read()? {
            Some(self.refs.read(&branch)?)
        } else {
            None
        };

        let commit = repository::Commit::new(previous_commit_oid, root_tree_oid, msg);
        let new_head_oid = self.db.store(&commit)?;
        //todo: change this part to use checkout command
        if let Some(branch) = self.head.read()? {
            self.refs.write(&branch, &new_head_oid)?;
        } else {
            let main = self.refs.init(&new_head_oid)?;
            self.head.write(main)?;
        }
        
        Ok(())
    }
}
