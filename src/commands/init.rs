pub struct Init {
    working_directory: std::path::PathBuf
}
impl Init {
    pub fn new<P: AsRef<std::path::Path>>(working_directory: P) -> std::io::Result<Self> {
        // compare to `absolute`, `canonicalize` checks symlink and so on,
        // to make sure the path exists
        let working_directory = std::path::Path::canonicalize(working_directory.as_ref())?;

        Ok(Self {
            working_directory
        })
    }

    pub fn run(&self) -> std::io::Result<()> {
        let repository_directory = self.working_directory.join(".rit");

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

        let repository_path = Repository::new(working_directory)?;
        let refs = Refs::new(repository_path)?;
        // todo: initialize HEAD to refs/heads/master


        println!("initialized repository!!");

        Ok(())
    }
}
