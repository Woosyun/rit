use std::{
    path::PathBuf,
    io,
};

pub struct Workspace {
    pub path: PathBuf
}
impl Workspace {
    pub fn new(path: PathBuf) -> Self {
        Self{ path }
    }

    // todo: make instance method? for what?
    pub fn list_files(path: PathBuf) -> io::Result<Vec<PathBuf>> {
        let mut re: Vec<Vec<PathBuf>> = vec![];
        for entry in path.read_dir()? {
            let path = entry?.path();
            if path.is_dir() {
                re.push(Workspace::list_files(path)?);
            } else {
                re.push(vec![path]);
            }
        }
        
        let re = re
            .into_iter()
            .flatten()
            .collect::<Vec<PathBuf>>();
        Ok(re)
    }

    pub fn get_relative_ancestors(&self, _target_path: PathBuf) -> Vec<PathBuf> {
        vec![]
    }

}
