use std::{path, fs, io};

pub struct Workspace {
    working_dir: path::PathBuf
}
impl Workspace {
    pub fn new<P: AsRef<path::Path>>(working_dir: P) -> io::Result<Self> {
        Ok(Self {
            working_dir: fs::canonicalize(working_dir.as_ref().to_path_buf())?
        })
    }

    /*
    pub fn list_all(&self) -> io::Result<Vec<path::Path>> {
        self.list_dir(self.working_dir)?

    }
    */
    fn list_dir<P: AsRef<path::Path>>(&self, dir: P) -> io::Result<Vec<path::PathBuf>> {
        let paths = fs::read_dir(dir)?
            .filter(|direntry| direntry.is_ok())
            .map(|direntry| direntry.unwrap().path())
            .collect::<Vec<path::PathBuf>>();

        Ok(paths)
    }
}
