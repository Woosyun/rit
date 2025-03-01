use crate::prelude::*;
use std::io;

impl Command {
    pub fn status(&self) -> io::Result<String> {
        let root = self.get_workdir();
    
        let result = Workspace::list_files(root)?
            .into_iter()
            .map(|file| file.to_str().unwrap().to_owned())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(result)
    }
}
