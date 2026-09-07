use crate::workspace::Workspace;
use std::{io, path};

pub struct Commit {
    working_directory: path::PathBuf,
    // ritignore
    // repository
}
impl Commit {
    pub fn new<P: AsRef<path::Path>>(working_directory: P) -> Self {
        Self {
            working_directory: working_directory.as_ref().to_path_buf(),
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        let root_path = self.working_directory.join(".rit");
        //let db_path = root_path.join("objects");

        let workspace = Workspace::new(&self.working_directory)?;
        let files = workspace.list_files()?;
        println!("{:?}", files);

        Ok(())
    }
}
