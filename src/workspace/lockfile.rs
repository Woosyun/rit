use std::path::Path;
use crate::fs;

pub fn write(file: &Path, content: &str) -> crate::Result<()> {
    let mut lockfile = file.to_path_buf();
    lockfile.set_extension("lock");

    fs::write(&lockfile, content)?;
    fs::rename(&lockfile, file)?;

    Ok(())
}
