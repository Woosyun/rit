use rit::commands::init::*;
use std::{fs, error, io, path};
use tempdir::TempDir;

fn check_repository<P: AsRef<path::Path>>(repo_dir: P) -> io::Result<()> {
    let repo_dir = repo_dir.as_ref();

    assert!(repo_dir.is_dir());
    assert!(repo_dir.join("objects").is_dir());
    assert!(repo_dir.join("refs").is_dir());
    // tothink: Is refs/heads default dirs?

    Ok(())
}

#[test]
pub fn init_succeed_on_empty_directory() -> Result<(), Box<dyn error::Error>> {
    let temp_dir = TempDir::new("init_succeed_on_empty_directory")?;
    Init::new(temp_dir.path())?
        .run()?;

    let repository_dir = temp_dir.path().join(REPOSITORY_DIR);
    check_repository(repository_dir)?;

    Ok(())
}

#[test]
pub fn init_pass_on_clean_repository() -> Result<(), Box<dyn error::Error>> {
    let temp_dir = TempDir::new("init_pass_on_clean_repository")?;
    let repo_dir = temp_dir.path().join(REPOSITORY_DIR);

    fs::create_dir(&repo_dir)?;
    fs::create_dir(repo_dir.join("objects"))?;
    fs::create_dir(repo_dir.join("refs"))?;

    /* tothink: Is HEAD default?
    fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(temp_dir.path().join(".rit/HEAD"))?
        .write_all(b"branch: refs/heads/main")?;
    */

    Init::new(temp_dir.path())?
        .run()?;
    check_repository(repo_dir)?;

    Ok(())
}
