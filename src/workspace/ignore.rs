use serde::{Serialize, Deserialize};
use serde_json;
use std::{
    fs,
    path::{PathBuf, Path},
    collections::HashSet,
};
use crate::prelude::*;


// .ignore file should be at workspace
// so ignore file can be stored in each revision

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Ignore{
    path: PathBuf,
    names: HashSet<Index>,
}
impl Ignore {
    pub fn name() -> &'static str {
        ".ritignore"
    }
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
    pub fn store(&self) -> Result<()> {
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
