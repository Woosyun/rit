use rit::prelude::*;
use std::{
    path::{Path, PathBuf},
    time::Duration,
    thread,
    fs,
};
use filetime::FileTime;

pub trait Driver {
    fn workdir(&self) -> &Path;
    fn workspace(&self) -> Result<Workspace> {
        Workspace::build(self.workdir().to_path_buf())
    }
    fn repository(&self) -> Result<Repository> {
        Repository::build(&self.workspace()?)
    }
    //to make mtime differ
    fn sleep_1_sec(&self) {
        thread::sleep(Duration::from_secs(1));
    }
    fn read_to_file(&self, index: &Path) -> Result<File> {
        let path = self.workdir().join(index);
        //1. oid
        let content = fs::read_to_string(&path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let blob = Blob::new(content);
        let blob_str = serde_json::to_string(&blob)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let oid = Oid::build(&blob_str);
        let metadata = fs::metadata(path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        let mtime = FileTime::from_last_modification_time(&metadata)
            .unix_seconds();
        Ok(File::from(index.to_path_buf(), oid, mtime))
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct File {
    index: PathBuf,
    oid: Oid,
    mtime: i64,
}
impl File {
    fn from(index: PathBuf, oid: Oid, mtime: i64) -> Self {
        Self {
            index,
            oid,
            mtime
        }
    }
}
