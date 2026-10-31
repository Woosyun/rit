//! # File-Write-Locking
//! Refer git/lockfile.h
//!
//! Mutual exclusion, atomic writing.
//! Reading is not gonna be blocked, if 'rename' is atomic.
//! When we write file, we create {file name}.lock file first,
//! write contents into it, rename file name to {file name}.
//!
//! With OptionOptions::new().create_new(true), we can notice
//! when there is somebody already locked.
//!
//! Even when process finish before LockFile is renamed,
//! cleaning up should be garanteed.
//!
//! But could drop trait be problematic,
//! considering drop trait return (),
//! it cannot handle errors, including disk error?
//!
//! ## Compare to OS file locking
//! ### OS File-Locking
//! pros: kernel level locking. automatic releasing
//! cons: behaviour depends on OS ( flock on Unix, LockFileEx on Windows)
//!
//! ### File existence locking
//! pros: true cross platform safety
//! cons: fragile

// tothink:
// can I pass responsibility of actual handling file
// to something like tempdir?

use std::{error, fs, io, path};

pub struct LockFile {
    /// Temporary path, "*.lock".
    path: path::PathBuf,

    /// Target file path.
    target_path: path::PathBuf,
    file: Option<fs::File>,
}
impl LockFile {
    pub fn new<P: AsRef<path::Path>>(path: P) -> LockFile {
        let lock_path = path.as_ref().with_added_extension("lock");
        Self {
            path: lock_path,
            target_path: path.as_ref().to_path_buf(),
            file: None,
        }
    }

    pub fn hold(&mut self) -> io::Result<()> {
        let file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(&self.path)?;
        self.file = Some(file);

        Ok(())
    }
    pub fn commit(mut self) -> Result<(), Box<dyn error::Error>> {
        if self.file.is_none() {
            return Err("forbidden operation".into());
        }

        // For Windows compatibility, drop file before renaming.
        drop(self.file.take());

        fs::rename(&self.path, &self.target_path).map_err(Into::into)
    }
}

impl io::Write for LockFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(f) = &mut self.file {
            return f.write(buf);
        }
        //means buffer is empty or file cannot accepts bytes anymore.
        Ok(0)
    }
    fn flush(&mut self) -> io::Result<()> {
        if let Some(f) = &mut self.file {
            return f.flush();
        }
        Ok(())
    }
}

impl io::Read for LockFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(f) = &mut self.file {
            return f.read(buf);
        }
        Ok(0)
    }
}

// For unexpected errors
impl Drop for LockFile {
    fn drop(&mut self) -> () {
        // todo: handle errors;;
        if self.path.exists() {
            let _ = fs::remove_file(&self.path);
        }
    }
}
