pub mod tree;
pub use tree::*;

pub mod stat;
pub use stat::*;

pub mod file;
pub use file::*;

pub mod ignore;
pub use ignore::*;

use std::{
    path::{PathBuf, Path},
    collections::HashMap,
    fs,
};
use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    workdir: PathBuf,
    //todo: ignore -> index, not file name
    ignore: Ignore,
}
impl Workspace {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        if !workdir.exists() {
            return Err(crate::Error::Workspace("NOT FOUND".into()));
        }
        let ignore = Ignore::load(workdir.clone())?;

        let ws = Self {
            workdir,
            ignore,
        };
        Ok(ws)
    }

    pub fn workdir(&self) -> &Path {
        &self.workdir
    }

    /*
    pub fn get_ancestors(&self, index: &Index) -> crate::Result<Vec<String>> {
        let mut ancestors = Vec::new();
        for p in index.ancestors() {
            if p.as_os_str().is_empty() {
                continue;
            }
            let ancestor = if let Some(ancestor) = p.file_name() {
                if let Some(ancestor) = ancestor.to_str() {
                    ancestor.to_string()
                } else {
                    let f = format!("Cannot stringify {:?}", index);
                    return Err(Error::Workspace(f));
                }
            } else {
                let f = format!("cannot get file name of {:?}", index);
                return Err(Error::Workspace(f));
            };
            ancestors.push(ancestor);
        }
        Ok(ancestors)
    }
    */

    pub fn get_relative_path(&self, path: &Path) -> crate::Result<Index> {
        path.strip_prefix(self.workdir())
            .map(|p| p.to_path_buf())
            .map_err(|e| {
                let f = format!("{:?}: {}", path, e);
                crate::Error::Workspace(f)
            })
    }

    pub fn list_files(&self, dir: &Path, rev: &mut HashMap<PathBuf, Box<dyn Stat>>) -> crate::Result<()> {
        let read_dir = fs::read_dir(dir)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        for entry in read_dir {
            let entry = entry
                .map_err(|e| Error::Workspace(e.to_string()))?;
            let path = entry.path();
            let index = self.get_relative_path(&path)?;

            if self.ignore.is_ignored(&index) {
                continue;
            }
            let file_type = entry.file_type()
                .map_err(|e| Error::Workspace(e.to_string()))?;

            if file_type.is_dir() {
                self.list_files(&path, rev)?;
            } else {
                let file = Box::new(File::build(&path)?);
                let _ = rev.insert(index, file);
            }
        }

        Ok(())
    }
}

impl IntoRev for Workspace {
    fn into_rev(&self) -> crate::Result<Rev> {
        let mut rev = HashMap::new();
        self.list_files(self.workdir(), &mut rev)?;
        Ok(Rev::from(rev))
    }
}

/*
impl HandleRevDiff for Workspace {
    fn handle_rev_diff(&self, source: impl HandleRevDiff, rev_diff: RevDiff) -> Result<()>;
}
*/

#[cfg(test)]
#[path = "./workspace.test.rs"]
mod test;
