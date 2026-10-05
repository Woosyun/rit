use std::{path, fs, io, collections};

pub struct Workspace {
    working_dir: path::PathBuf
}
impl Workspace {
    pub fn new<P: AsRef<path::Path>>(working_dir: P) -> io::Result<Self> {
        Ok(Self {
            working_dir: fs::canonicalize(working_dir.as_ref().to_path_buf())?
        })
    }

    pub fn list_files(&self) -> io::Result<Vec<path::PathBuf>> {
        let mut hash = vec![];
        self.read_dir(&self.working_dir, &mut hash)?;

        Ok(hash)
    }
    fn read_dir<P: AsRef<path::Path>>(&self, dir: P, hash: &mut Vec<path::PathBuf>) -> io::Result<()> {
        for direntry in fs::read_dir(&dir)? {
            let direntry = direntry?;
            let entry_name = direntry.path();
            let target_path = dir.as_ref().join(entry_name);

            //todo: filter with .gitignore

            if direntry.file_type()?.is_dir() {
                self.read_dir(target_path, hash)?;
            } else {
                hash.push(target_path);
            }
        }

        Ok(())
    }
}
