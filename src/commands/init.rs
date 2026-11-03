use crate::prelude::*;
use std::{error, fs, io, path};

/// A Command for initializing repository.
/// ## Usage
///
/// ```
/// Init::builder()
///     .working_directory(env::current_dir()?)
///     .build()
///     .execute()?
/// ```
///
pub struct Init {
    repository: Repository,
}
impl Command<Init, InitBuilder> for Init {
    fn builder() -> InitBuilder {
        InitBuilder::new()
    }
    fn execute(self) -> Result<(), Box<dyn error::Error>> {
        match fs::create_dir(&self.repository.repodir) {
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
            let target_path = self.repository.repodir.join(sub_dir);
            fs::create_dir(target_path)?
        }

        Ok(())
    }
}

/// Builder for [`Init`]
pub struct InitBuilder {
    working_directory: Option<path::PathBuf>,
}
impl CommandBuilder<Init> for InitBuilder {
    fn new() -> Self {
        Self {
            working_directory: None,
        }
    }
    fn working_directory<P: AsRef<path::Path>>(mut self, workdir: P) -> Self {
        self.working_directory = Some(workdir.as_ref().to_path_buf());
        self
    }
    fn build(self) -> io::Result<Init> {
        let Some(working_directory) = self.working_directory else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Working directory not found",
            ));
        };
        let repository = Repository::build(working_directory)?;
        Ok(Init { repository })
    }
}
