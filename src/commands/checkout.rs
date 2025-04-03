use crate::{workspace, repository, commands::Status};
use std::path::PathBuf;

pub struct Checkout {
    ws: workspace::Workspace,
    db: repository::Database,
    head: repository::Head,
    refs: repository::Refs,
    current_position: repository::Oid,
}

impl Checkout {
    pub fn build(cwd: PathBuf) -> crate::Result<Self> {
        let mut status = Status::build(cwd.clone())?;
        status.execute()?;
        if !status.is_clean() {
            return Err(crate::Error::Workspace("state of workspace is not clean".into()));
        }

        let ws = workspace::Workspace::build(cwd.clone())?;
        let repo = repository::Repository::build(&ws)?;
        let db = repo.get_database()?;
        let head = repo.get_head();
        let refs = repo.get_refs();
        let current_position = match head.read()? {
            Some(branch) => refs.read(&branch)?,
            None => return Err(crate::Error::Repository("history not found".into())),
        };

        let re = Self {
            ws,
            db,
            head,
            refs,
            current_position,
        };

        Ok(re)
    }

    pub fn up(&mut self) -> crate::Result<Option<()>> {
        let cur_commit: repository::Commit = self.db.retrieve(&self.current_position)?;

        if let Some(parent) = cur_commit.parent() {
            self.current_position = parent.clone();
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    //todo: implement
    pub fn apply(&self) -> crate::Result<()> {
        Ok(())
    }
}
