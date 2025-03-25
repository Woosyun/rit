use std::{
    fs,
    path::Path,
};


pub fn read_to_string(path: &Path) -> crate::Result<String> {
    fs::read_to_string(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn write(path: &Path, content: &str) -> crate::Result<()> {
    fs::write(path, content)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn read_dir(path: &Path) -> crate::Result<fs::ReadDir> {
    fs::read_dir(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn metadata(path: &Path) -> crate::Result<fs::Metadata> {
    fs::metadata(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn create_dir(path: &Path) -> crate::Result<()> {
    fs::create_dir(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}
pub fn create_dir_all(path: &Path) -> crate::Result<()> {
    fs::create_dir_all(path)
        .map_err(|e| {
            let msg = format!("{:?}: {}", path, e);
            crate::Error::Io(msg)
        })
}

pub fn rename(base: &Path, target: &Path) -> crate::Result<()> {
    fs::rename(base, target)
        .map_err(|e| {
            let msg = format!("{:?}->{:?}: {}", base, target, e);
            crate::Error::Io(msg)
        })
}
