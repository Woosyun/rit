use crate::prelude::*;
use std::{error, fs, io, path};

pub struct Builder {
    working_directory: Option<path::PathBuf>,
}
impl Builder {
    pub fn new() -> Self {
        Self {
            working_directory: None,
        }
    }
    pub fn working_directory<P: AsRef<path::Path>>(mut self, workdir: P) -> Self {
        self.working_directory = Some(workdir.as_ref().to_path_buf());
        self
    }
    pub fn build(self) -> io::Result<Command> {
        let Some(working_directory) = self.working_directory else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Working directory not found",
            ));
        };
        let repository = Repository::build(working_directory)?;
        Ok(Command { repository })
    }
}

pub struct Command {
    repository: Repository,
}
impl Command {
    pub fn execute(self) -> Result<(), Box<dyn error::Error>> {
        match fs::create_dir(&self.repository.path) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                io::ErrorKind::AlreadyExists => {
                    return Ok(());
                }
                _ => {
                    return Err(error.into());
                }
            },
        }

        for sub_dir in ["objects", "refs"] {
            let target_path = self.repository.path.join(sub_dir);
            fs::create_dir(target_path)?
        }

        Ok(())
    }
}
