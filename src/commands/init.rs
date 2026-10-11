use crate::repository;
use std::{fs, io, path};

pub struct Init {
    working_directory: path::PathBuf,
}
impl Init {
    // responsible getting working directory right
    pub fn new<P: AsRef<path::Path>>(working_directory: P) -> io::Result<Self> {
        // compare to `absolute`, `canonicalize` checks symlink and so on,
        // to make sure the path exists
        let working_directory = path::Path::canonicalize(working_directory.as_ref())?;

        Ok(Self { working_directory })
    }

    pub fn run(&self) -> std::io::Result<()> {
        let repository_directory = self.working_directory.join(repository::DEFAULT_NAME);

        match fs::create_dir(&repository_directory) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                io::ErrorKind::AlreadyExists => {
                    return Ok(());
                }
                _ => {
                    return Err(error);
                }
            },
        }

        for sub_dir in repository::DEFAULT_SUBS {
            let target_path = repository_directory.join(sub_dir);

            fs::create_dir(target_path)?
        }

        Ok(())
    }
}
