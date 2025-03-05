use crate::prelude::*;

impl Command {
    pub fn commit(&self, message: String) -> std::io::Result<String> {
        let db = self.get_repository()?
            .get_database()?;

        let root = self.get_workdir();
        let mut tree = Tree::new();
        
        for file in Workspace::list_files(root)? {
            let oid = self.workspace.read_file(file.clone())
                .map(|content| db.store(&Blob::new(content)))??;
            let metadata = self.workspace.read_metadata(file.clone())?;
            let entry = Entry::new(EntryKind::Blob, oid, /*name*/);

            // todo: crate ancestors
            //let ancestors = self.workspace.get_ancestors(file);
            let ancestors = self.workspace.get_ancestors(file);
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
