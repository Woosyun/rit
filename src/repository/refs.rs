use std::path::PathBuf;
use crate::{
    repository::database::Oid,
    workspace::lockfile,
    utils,
    fs,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Refs {
    path: PathBuf,
}
impl Refs {
    pub fn name() -> &'static str {
        "refs"
    }
    pub fn local() -> &'static str {
        "heads"
    }
    pub fn main() -> &'static str {
        "main"
    }
    pub fn new(repo: PathBuf) -> Self {
        let mut path = repo;
        path.push(Refs::name());

        Self {
            path
        }
    }
    pub fn init(&self, oid: &Oid) -> crate::Result<&'static str> {
        self.write(Refs::main(), oid)?;
        Ok(Refs::main())
    }

    pub fn read(&self, branch: &str) -> crate::Result<Oid> {
        let mut path = self.path.clone();
        path.push(Refs::local());
        path.push(branch);
        if !path.exists() {
            let msg = format!("branch {} not found", branch);
            return Err(crate::Error::Repository(msg));
        }

        let lastest_commit: Oid = fs::read_to_string(&path)
            .map(|content| {
                utils::encode(&content)
                    .map_err(|e| crate::Error::Repository(e))
            })??;

        Ok(lastest_commit)
    }

    pub fn write(&self, branch: &str, oid: &Oid) -> crate::Result<()> {
        let mut path = self.path.clone();
        path.push(Refs::local());
        fs::create_dir_all(&path)?;
        path.push(branch);

        let oid = utils::decode(oid)
            .map_err(|e| crate::Error::Repository(e))?;
        lockfile::write(&path, &oid)?;
        Ok(())
    }
}
