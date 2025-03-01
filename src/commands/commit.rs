use crate::prelude::*;
use std::{
    io,
};

impl Command {
    pub fn commit(&self, message: String) -> io::Result<String> {
        let db = self.get_repository()?
            .get_database()?;

        let root = self.get_workdir();
        let mut tree = Entry::build_tree(root.clone());
        
        for file in Workspace::list_files(root)? {
            let content = String::new();
            // todo: read file from path

            let blob = Blob::new(content);
            let hash = db.store(&blob)?;

            let entry = Entry::build_blob(file, Some(hash));
            // todo: crate ancestors
            //let ancestors = self.workspace.get_ancestors(file);
            let ancestors = entry.path.clone();
            let ancestors = ancestors.ancestors()
                .into_iter()
                .collect::<Vec<_>>();
            tree.add_entry(ancestors, entry);
        }

        tree.traverse_mut(|tree: &mut Entry| {
            let hash = tree.calculate_hash();
            tree.hash = Some(hash);
        });

        tree.traverse(|tree: &Entry| {
            let _ = db.store(tree);
        });

        //store commit and update head
        let commit = Commit::build(message, "author".to_string(), "commiter".to_string());
        let hash = db.store(&commit)?;
        let return_msg = format!("commit id: {}", String::from(hash));

        Ok(return_msg)
    }
}
