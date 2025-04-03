pub mod tree;
pub use tree::*;

pub mod stat;
pub use stat::*;

pub mod lockfile;

use std::{
    path::{PathBuf, Path},
};
use crate::{
    repository,
    fs,
};

#[derive(PartialEq, Clone, Debug)]
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

    //todo: delete this function. make models as simple as possible
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

    pub fn read_dir(&self, path: &Path) -> crate::Result<Vec<PathBuf>> {
        let mut paths = vec![];
        for entry in fs::read_dir(path)? {
            if let Ok(entry) = entry {
                paths.push(entry.path());
            }
        }
        Ok(paths)
    }

    pub fn read_stat(&self, file: &Path) -> crate::Result<Stat> {
        let stat = Stat::from_metadata(&fs::metadata(file)?);
        Ok(stat)
    }
    
    pub fn read_to_blob(&self, file: &Path) -> crate::Result<repository::Blob> {
        let content = fs::read_to_string(file)?;
        let blob = repository::Blob::new(content);
        Ok(blob)
    }

    // path should not be relative path
    pub fn get_ancestors(&self, path: &Path) -> crate::Result<Vec<String>> {
        let relative_path = self.get_relative_path(path)?;
        let mut ancestors = relative_path.ancestors()
            .collect::<Vec<_>>();
        ancestors.pop();
        let ancestors = ancestors
            .into_iter()
            .map(|p| self.get_file_name(p))
            .collect::<crate::Result<Vec<_>>>()?;
        Ok(ancestors)
    }

    pub fn get_file_name(&self, path: &Path) -> crate::Result<String> {
        match path.file_name() {
            Some(file_name) => {
                let file_name = file_name
                    .to_str().unwrap()
                    .to_string();
                Ok(file_name)
            },
            None => {
                let f = format!("{:?}: cannot get file name", path);
                Err(crate::Error::Workspace(f))
            }
        }
    }
    pub fn get_relative_path(&self, path: &Path) -> crate::Result<PathBuf> {
        path.strip_prefix(&self.path)
            .map(|p| p.to_path_buf())
            .map_err(|e| crate::Error::Workspace(e.to_string()))
    }
}
