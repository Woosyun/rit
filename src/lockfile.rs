//
// Think about file-existence locking vs os-file-locking
// and replace this method to OS level file locking method someday
//
// File-existence locking
// pros: true cross platform safety
// cons: fragile
//
// OS-file locking
// pros: kernel level locking. automatic releasing
// cons: behaviour depends on OS ( flock on Unix, LockFileEx on Windows)
//
// And for releasing locking,
// use explicit release method, not through drop trait.
// since drop returns (), it cannot handle errors, including disk error
//

use std::{fs, io, path};
use std::io::prelude::*;

pub struct LockFile {
    path: path::PathBuf,
    target_path: path::PathBuf,
    _file: Option<fs::File>,
}
impl LockFile {
    pub fn new<P: AsRef<path::Path>>(path: P) -> LockFile {
        let lock_path = path.as_ref().with_added_extension("lock");
        Self {
            path: lock_path,
            target_path: path.as_ref().to_path_buf(),
            _file: None,
        }
    }

    pub fn load(&mut self) -> io::Result<()> {
        let file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create_new(true)
            .open(&self.path)?;
        self._file = Some(file);

        Ok(())
    }
    pub fn load_mut(&mut self) -> io::Result<()> {
        let file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(&self.path)?;
        self._file = Some(file);

        Ok(())
    }
    pub fn write(&mut self, content: &[u8]) -> io::Result<()> {
        if let Some(f) = &mut self._file {
            f.write_all(content)?;
        }
        Ok(())
    }
    pub fn commit(mut self) -> io::Result<()> {
        // For Windows compatibility, drop file before renaming.
        drop(self._file.take());

        fs::rename(&self.path, &self.target_path)
    }
}

// For unexpected errors
impl Drop for LockFile {
    fn drop(&mut self) -> () {
        // todo: handle errors;;
        let _ = fs::remove_file(&self.path);
    }
}

/*
impl std::ops::Deref for LockFile {
    type Target = Path;
    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
*/
