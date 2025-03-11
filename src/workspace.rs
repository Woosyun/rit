use crate::prelude::*;
use std::{
    path::{PathBuf, Path},
    io,
    fs,
};

pub struct Workspace {
    path: PathBuf,
}
impl Workspace {
    pub fn build(path: PathBuf) -> io::Result<Self> {
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "workspace not found"));
        }

        let ws = Self {
            path
        };

        Ok(ws)
    }

    pub fn list_files(&self, path: Option<PathBuf>) -> io::Result<Vec<PathBuf>> {
        let path = match path {
            Some(path) => path,
            None => self.path.clone(),
        };

        let mut files = vec![];
        for entry in fs::read_dir(path)? {
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

    pub fn read_to_blob(&self, file: &Path) -> io::Result<Blob> {
        let content = fs::read_to_string(file)?;
        Ok(Blob::new(content))
    }

    pub fn ancestors(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        let root = self.path.clone();
        let rel = path.strip_prefix(root)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        let mut ancestors = rel.ancestors()
            .map(|p| p.to_path_buf())
            .collect::<Vec<_>>();
        let _ = ancestors.pop();
        Ok(ancestors)
    }
}
