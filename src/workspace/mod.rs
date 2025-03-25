pub mod tree;
pub use tree::*;

pub mod stat;
pub use stat::*;

pub mod lockfile;

use std::{
    path::{PathBuf, Path},
};
use crate::{
    repository::database,
    fs,
};

pub struct Workspace {
    pub path: PathBuf,
}
impl Workspace {
    pub fn build(path: PathBuf) -> crate::Result<Self> {
        if !path.exists() {
            return Err(crate::Error::Workspace("workspace not found".into()));
        }

        let ws = Self {
            path
        };

        Ok(ws)
    }

    pub fn list_files(&self, path: Option<PathBuf>) -> crate::Result<Vec<PathBuf>> {
        let path = match path {
            Some(path) => path,
            None => self.path.clone(),
        };

        let mut files = vec![];
        for entry in fs::read_dir(&path)? {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        files.push(vec![entry.path()]);
                    } else if let Ok(sub_files) = self.list_files(Some(entry.path())) {
                        files.push(sub_files);
                    } else {
                        files.push(vec![]);
                    }
                }
            }
        }

        let files = files
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        Ok(files)
    }

    pub fn read_stat(&self, file: &Path) -> crate::Result<Stat> {
        let stat = Stat::from_metadata(fs::metadata(file)?);
        Ok(stat)
    }
    
    pub fn read_to_blob(&self, file: &Path) -> crate::Result<database::Blob> {
        let content = fs::read_to_string(file)?;
        let blob = database::Blob::new(content);
        Ok(blob)
    }

    pub fn ancestors(&self, path: &Path) -> crate::Result<Vec<PathBuf>> {
        let root = self.path.clone();
        let rel = path.strip_prefix(root)
            .map_err(|e| crate::Error::Workspace(e.to_string()))?;
        let mut ancestors = rel.ancestors()
            .map(|p| p.to_path_buf())
            .collect::<Vec<_>>();
        let _ = ancestors.pop();
        Ok(ancestors)
    }
}
