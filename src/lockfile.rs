// think about file-existence locking vs os-file-locking
// and replace this method to OS level file locking method someday
//
// file-existence locking
// pros: true cross platform safety
// cons: fragile
//
// os-file locking
// pros: kernel level locking. automatic releasing
// cons: behaviour depends on OS ( flock on Unix, LockFileEx on Windows)
//
// and for releasing locking,
// use explicit release method, not through drop trait.
// since drop returns (), it cannot handle errors, including disk error

use std::{
    path::{PathBuf, Path},
    fs,
    io,
};

pub struct LockFile {
    path: PathBuf
}
impl LockFile {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<LockFile> {
        let path = path.as_ref();
        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)?;
        Ok(Self {
            path: path.to_path_buf()
        })
    }
    pub fn commit(self) -> io::Result<()> {
        todo!("rename")
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl std::ops::Deref for LockFile {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
