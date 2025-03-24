use std::{
    path::Path,
    fs,
    io,
};

pub fn store(path: &Path, content: String) -> io::Result<()> {
    let mut lockfile = path.to_path_buf();
    lockfile.push("lock");

    let _ = fs::write(&lockfile, content)?;
    let _ = fs::rename(&lockfile, path)?;

    Ok(())
}

