use crate::prelude::*;

pub fn init(path: std::path::PathBuf) -> std::io::Result<&'static str> {
    Repository::init(path)
}
