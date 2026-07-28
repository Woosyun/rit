pub struct Init {
    working_directory: std::path::PathBuf
}
impl Init {
    pub fn new<P: AsRef<std::path::Path>>(working_directory: P) -> std::io::Result<Self> {
        // compare to `absolute`, `canonicalize` check symlink and so on to provide existence of
        // the target path
        let working_directory = std::path::Path::canonicalize(working_directory.as_ref())?;

        Ok(Self {
            working_directory
        })
    }

    pub fn run(&self) -> std::io::Result<()> {
        let repository_directory = self.working_directory.join(".git");

        match std::fs::create_dir(&repository_directory) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => { return Ok(()); },
                _ => { return Err(error); },
            }
        }

        let _ = ["objects", "refs"].iter().map(|sub_dir| {
            let target_path = repository_directory.join(sub_dir);

            std::fs::create_dir(target_path)
        }).collect::<std::io::Result<Vec<_>>>()?;

        println!("initialized empty repository!!");

        Ok(())
    }
}
