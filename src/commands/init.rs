use crate::commands::command::*;
use crate::repository;
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
    working_directory: path::PathBuf,
}
impl Command<Init, InitBuilder> for Init {
    fn builder() -> InitBuilder {
        InitBuilder::new()
    }
    fn execute(self) -> Result<(), Box<dyn error::Error>> {
        let repository_directory = self.working_directory.join(repository::DIR_NAME);

        match fs::create_dir(&repository_directory) {
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

        for sub_dir in ["objects"] {
            let target_path = repository_directory.join(sub_dir);
            fs::create_dir(target_path)?
        }

        Ok(())
    }
}

/// Builder for [`Init`]
pub struct InitBuilder {
    working_directory: Option<path::PathBuf>,
}
impl InitBuilder {
    pub fn working_directory<P: AsRef<path::Path>>(mut self, workdir: P) -> Self {
        self.working_directory = Some(workdir.as_ref().to_path_buf());
        self
    }
}
impl CommandBuilder<Init> for InitBuilder {
    fn new() -> Self {
        Self {
            working_directory: None,
        }
    }
    fn build(self) -> Result<Init, Box<dyn error::Error>> {
        let Some(working_directory) = self.working_directory else {
            return Err("no working_directory".into());
        };
        Ok(Init { working_directory })
    }
}
