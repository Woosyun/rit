use crate::prelude::*;
use std::{
    io,
    fs,
};

impl Command {
    pub fn init(&self) -> io::Result<String> {
        match self.get_repository() {
            Ok(_) => {
                Ok("repository already exists".to_string())
            },
            _ => {
                let mut root = self.get_workdir();
                root.push(".rit");

                fs::create_dir(root)
                    .map(|_| "repository created".to_string())
            }
        }
    }
}
