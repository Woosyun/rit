use std::{path, io, fs};

pub const REPOSITORY_DIR: &'static str = ".rit";
pub const DEFAULT_SUB_DIRS: [&'static str; 2]= [
    "objects",
    "refs",
];

pub struct Init {
    working_directory: path::PathBuf
}
impl Init {
    // responsible for get the path right
    pub fn new<P: AsRef<path::Path>>(working_directory: P) -> io::Result<Self> {
        // compare to `absolute`, `canonicalize` checks symlink and so on,
        // to make sure the path exists
        let working_directory = path::Path::canonicalize(working_directory.as_ref())?;

        Ok(Self {
            working_directory
        })
    }

    pub fn run(&self) -> std::io::Result<()> {
        let repository_directory = self.working_directory.join(REPOSITORY_DIR);

        match fs::create_dir(&repository_directory) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                io::ErrorKind::AlreadyExists => { return Ok(()); },
                _ => { return Err(error); },
            }
        }

        for sub_dir in DEFAULT_SUB_DIRS {
            let target_path = repository_directory.join(sub_dir);

            fs::create_dir(target_path)?
        }

        Ok(())
    }
}
