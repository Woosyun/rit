use serde::{Serialize, Deserialize};
use serde_json;
use std::{
    fs,
    path::PathBuf,
    collections::HashSet,
};
use crate::prelude::*;


// .ignore file should be at workspace
// so ignore file can be stored in each revision

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Ignore{
    names: HashSet<Index>,

    #[serde(skip)]
    path: PathBuf,
}
impl Ignore {
    pub fn name() -> &'static str {
        ".ritignore"
    }
    /*
    fn new(path: PathBuf) -> Self {
        let rit = Repository::name();
        let rit = Path::new(rit).to_path_buf();
        Self {
            path,
            names: HashSet::from([rit]),
        }
    }
    pub fn build(path: PathBuf) -> crate::Result<Self> {
        let mut path = path;
        path.push(Ignore::name());
        if !path.exists() {
            return Ok(Self::new(path));
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Ignore(e.to_string()))?;
        let ignore: Ignore = serde_json::from_str(&content)
            .map_err(|e| Error::Ignore(e.to_string()))?;
        Ok(ignore)
    }
    */
    pub fn load(workdir: impl Into<PathBuf>) -> Result<Self> {
        let mut path: PathBuf = workdir.into();
        path.push(Ignore::name());

        if path.exists() {
            let content = fs::read_to_string(path)
                .map_err(|e| Error::Ignore(e.to_string()))?;
            let ignore: Ignore = serde_json::from_str(&content)
                .map_err(|e| Error::Ignore(e.to_string()))?;
            Ok(ignore)
        } else {
            Ok(Ignore {
                path,
                names: HashSet::new(),
            })
        }
    }
    pub fn save(&self) -> Result<()> {
        let rit_ignore = serde_json::to_string(self)
            .map_err(|e| Error::Ignore(e.to_string()))?;
        fs::write(&self.path, &rit_ignore)
            .map_err(|e| Error::Ignore(e.to_string()))
    }

    pub fn add(&mut self, index: Index) {
        self.names.insert(index);
    }
    pub fn is_ignored(&self, index: &Index) -> bool {
        self.names.contains(index)
    }
}

impl Drop for Ignore {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("Failed to save ignore file: {e}");
        }
    }
}

#[cfg(test)]
#[path = "./ignore.test.rs"]
mod test;
