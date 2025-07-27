use serde::{Serialize, Deserialize};
use serde_json;
use std::{
    fs,
    path::{PathBuf, Path},
    collections::HashSet,
};
use crate::prelude::*;

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
    pub fn load(workdir: impl Into<PathBuf>) -> Result<Self> {
        let mut path: PathBuf = workdir.into();
        path.push(Ignore::name());

        if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| Error::Ignore(e.to_string()))?;
            let names: HashSet<PathBuf> = serde_json::from_str(&content)
                .map_err(|e| Error::Ignore(e.to_string()))?;
            Ok(Ignore {
                path,
                names,
            })
        } else {
            Ok(Ignore {
                path,
                names: HashSet::from([Path::new(".rit").into()]),
            })
        }
    }
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string(&self.names)
            .map_err(|e| Error::Ignore(e.to_string()))?;
        fs::write(&self.path, content)
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
