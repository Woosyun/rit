use std::{
    path::PathBuf,
    io,
};

pub struct Workspace {
    pub path: PathBuf
}
impl Workspace {
    pub fn build(path: PathBuf) -> io::Result<Self> {
        if !path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Uninitialized"));
        }
        
        Ok(Self{path})
    }

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
}
