use serde::{Serialize, Deserialize};
use std::{
    path::{Path, PathBuf},
    fs,
};
use crate::prelude::*;
use filetime::FileTime;

pub type Index = PathBuf;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct File {
    name: String,
    oid: Option<Oid>,
    mtime: Mtime,
    mode: Mode,
}
impl File {
    pub fn build(path: &Path) -> crate::Result<Self> {
        if path.is_dir() {
            let f = format!("Trying to build File object from directory {:?}", path);
            return Err(Error::Workspace(f));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let mtime = FileTime::from_last_modification_time(&metadata)
            .unix_seconds();
        let mode = match metadata.permissions().readonly() {
            true => READONLY_FILE_MODE,
            _ => EXECUTABLE_FILE_MODE,
        };

        let name = match path.file_name() {
            Some(oss) => {
                match oss.to_str() {
                    Some(str) => str.to_string(),
                    None => {
                        let f = format!("Error while stringify {:?}", path);
                        return Err(Error::Workspace(f));
                    }
                }
            },
            None => {
                let f = format!("Cannot get file name from {:?}", path);
                return Err(Error::Workspace(f));
            }
        };

        let re = Self {
            name,
            oid: None,
            mtime,
            mode
        };
        Ok(re)
    }
    
}

impl Stat for File {
    fn mtime(&self) -> Mtime {
        self.mtime
    }
    fn mode(&self) -> Mode {
        self.mode
    }
    fn oid(&self) -> crate::Result<&Oid> {
        match &self.oid {
            Some(oid) => Ok(oid),
            None => Err(crate::Error::Workspace("use of oid() of File before set".into()))
        }
    }
    fn set_oid(&mut self, oid: Oid) {
        self.oid = Some(oid);
    }
    fn name(&self) -> &Name {
        &self.name
    }
    /*
    fn clone_box(&self) -> Box<dyn Stat> {
        Box::new(self.clone())
    }
    */
}
