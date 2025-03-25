use std::path::PathBuf;
use crate::{
    repository::database::Oid,
    workspace::lockfile,
    utils,
    fs,
};

pub const REFS: &str = "refs";
pub const HEADS: &str = "heads";

pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn new(repo: PathBuf) -> Self {
        let mut path = repo;
        path.push(REFS);

        Self {
            path
        }
    }
    pub fn init(&self, oid: &Oid) -> crate::Result<()> {
        self.write("main", oid)
    }

    pub fn read(&self, branch: &str) -> crate::Result<Oid> {
        let mut path = self.path.clone();
        path.push(HEADS);
        path.push(branch);
        if !path.exists() {
            let msg = format!("branch {} not found", branch);
            return Err(crate::Error::Repository(msg));
        }

        let lastest_commit: Oid = fs::read_to_string(&path)
            .map(|content| {
                utils::encode(&content)
                    .map_err(|e| {
                        let msg = format!("cannot encode lastest commit oid: {}", e);
                        crate::Error::Repository(msg)
                    })
            })??;

        Ok(lastest_commit)
    }

    pub fn write(&self, branch: &str, oid: &Oid) -> crate::Result<()> {
        let mut path = self.path.clone();
        path.push(HEADS);
        fs::create_dir_all(&path)?;
        //path.push(branch);

        utils::decode(oid)
            .map_err(|e| {
                let msg = format!("cannot decode oid: {}", e);
                crate::Error::Repository(msg)
            })
            .map(|content| lockfile::write(&path, branch, &content))??;
        Ok(())
    }
}
