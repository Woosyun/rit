use crate::prelude::*;
use std::{
    path::{PathBuf, self},
    io,
};

pub struct Command {
    pub workspace: Workspace,
}

impl Command {
    pub fn build(path: PathBuf) -> io::Result<Self> {
        let path = path::absolute(path)?;

        let workspace = Workspace::new(path);
        let re = Self {
            workspace,
        };

        Ok(re)
    }

    pub fn get_repository(&self) -> io::Result<Repository> {
        let mut path = self.get_workdir();
        path.push(".rit");

        Repository::build(path)
    }

    pub fn get_workdir(&self) -> PathBuf {
        self.workspace.path.clone()
    }
}

