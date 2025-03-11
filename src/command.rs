use std::{
    path::PathBuf,
    io,
};
use crate::prelude::*;

pub struct Command {
    pub repo: Repository,
    pub ws: Workspace,
}
impl Command {
    pub fn build(workdir: PathBuf) -> io::Result<Self> {
        let ws = Workspace::build(workdir.clone())?;
        let repo = Repository::build(workdir)?;

        let cmd = Self {
            repo,
            ws,
        };

        Ok(cmd)
    }

    pub fn init(workdir: PathBuf) -> io::Result<&'static str> {
        Repository::init(workdir)
    }

    pub fn commit(&self, _msg: String) -> io::Result<String> {
        let files = self.ws.list_files(None)?;

        let mut tree = Tree::new();
        for file in files {
            let blob = self.ws.read_to_blob(file.as_path())?;
            let oid = self.repo.store(&blob)?;

            let ancestors = self.ws.ancestors(&file)?;
            tree.add_entry(ancestors, oid);
        }

        let _ = tree.traverse_mut(|tree: &mut Tree| {
            let oid = self.repo.store(tree)?;
            tree.oid = Some(oid);

            Ok(())
        })?;

        let root_oid = tree.oid.unwrap();
        let commit = Commit::new("author".to_string(), root_oid, "author".to_string());
        let commit_oid = self.repo.store(&commit)?;

        let result = format!("commit is stored and id is {}", commit_oid.into_string());
        Ok(result)
    }
}
