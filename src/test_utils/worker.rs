use std::path::{PathBuf, Path};
use rand::prelude::*;

pub struct Worker {
    workdir: PathBuf,
}
impl Worker {
    pub fn from(workdir: PathBuf) -> Self {
        Self {
            workdir
        }
    }

    pub fn random_file_name(&self) -> PathBuf {
        let rnd = rand::rng()
            .random::<u64>();

        let file_name = format!("file_{}.txt", rnd);
        Path::new(&file_name).into()
    }
}
