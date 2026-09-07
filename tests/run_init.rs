use rit::commands::init::Init;
use tempdir::TempDir;
use std::{fs, error, io::Write};

#[test]
pub fn init_succeed_on_empty_directory() -> Result<(), Box<dyn error::Error>> {
    let temp_dir = TempDir::new("init_succeed_on_empty_directory")?;
    Init::new(temp_dir.path())?
        .run()?;

    assert!(temp_dir.path().join(".rit").is_dir());
    assert!(temp_dir.path().join(".rit/objects").is_dir());
    assert!(temp_dir.path().join(".rit/refs/heads").is_dir());
    assert!(temp_dir.path().join(".rit/HEAD").is_file());

    Ok(())
}

#[test]
pub fn init_pass_on_clean_repository() -> Result<(), Box<dyn error::Error>> {
    let temp_dir = TempDir::new("init_pass_on_clean_repository")?;

    fs::create_dir(temp_dir.path().join(".rit"))?;
    fs::create_dir(temp_dir.path().join(".rit/objects"))?;
    fs::create_dir(temp_dir.path().join(".rit/refs"))?;
    fs::create_dir(temp_dir.path().join(".rit/refs/heads"))?;
    fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(temp_dir.path().join(".rit/HEAD"))?
        .write_all(b"branch: refs/heads/main")?;

    Init::new(temp_dir.path())?
        .run()?;

    Ok(())
}

/*
#[test]
pub fn init_pass_on_broken_repository_1() -> Result<(), Box<dyn error::Error>> {
    let temp_dir = TempDir::new("init_pass_on_broken_repository")?;

    fs::create_dir(temp_dir.path().join(".rit"))?;
    fs::create_dir(temp_dir.path().join(".rit/objects"))?;
    fs::create_dir(temp_dir.path().join(".rit/refs"))?;
    fs::File::create_new(temp_dir.path().join(".rit/HEADS"))?;

    let init = Init::new(temp_dir.path())?;
    init.run()?;

    Ok(())
}

/// # Error
/// 
/// init fails on under situations
/// * 
#[test]
pub fn init_fails_on_broken_repository() -> Result<(), Box<dyn error::Error>> {
}
*/
