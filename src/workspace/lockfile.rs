use std::path::Path;
use crate::fs;

pub fn write(dir: &Path, file: &str, content: &str) -> crate::Result<()> {
    let mut lockfile = dir.to_path_buf();
    lockfile.push("lock");

    let _ = fs::write(&lockfile, content)?;

    let mut target = dir.to_path_buf();
    target.push(file);
    let _ = fs::rename(&lockfile, &target)?;

    Ok(())
}
