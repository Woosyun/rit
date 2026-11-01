use rit::commands::Init;
use rit::prelude::*;
use std::error;
use tempdir::TempDir;

#[test]
pub fn init_succeed_on_empty_directory() -> Result<(), Box<dyn error::Error>> {
    let tempdir = TempDir::new("init_succeed_on_empty_directory")?;
    Init::builder()
        .working_directory(tempdir.path())
        .build()?
        .execute()?;

    let repodir = tempdir.path().join(repository::REPOSITORY);
    assert!(repodir.exists());
    assert!(repodir.join(repository::DATABASE).exists());
    assert!(repodir.join(repository::REFS).exists());

    Ok(())
}
