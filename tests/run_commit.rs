use rit::prelude::*;
use std::{error, fs};
use tempdir::TempDir;

#[test]
#[should_panic(expected = "cannot run commit without repository initialized")]
fn commit_without_repository_initialized() -> () {
    let tempdir =
        TempDir::new("commit_on_empty_working_directory").expect("TempDir should be created");
    Commit::builder()
        .working_directory(tempdir.path())
        .message("no-commit")
        .author("me", "me123@abc.com")
        .committer("me", "me123@abc.com")
        .build()
        .expect("CommitBuilder should be able to build Commit")
        .execute()
        .expect("cannot run commit without repository initialized")
}

#[test]
fn commit_once() -> Result<(), Box<dyn error::Error>> {
    let tempdir = TempDir::new("commit_on_empty_working_directory")?;

    Init::builder()
        .working_directory(tempdir.path())
        .build()?
        .execute()?;

    fs::write(tempdir.path().join("a.txt"), "a")?;
    fs::create_dir(tempdir.path().join("b"))?;
    fs::write(tempdir.path().join("b/c.txt"), "c")?;

    Commit::builder()
        .working_directory(tempdir.path())
        .message("no-commit")
        .author("me", "me123@abc.com")
        .committer("me", "me123@abc.com")
        .build()?
        .execute()?;

    //needs status?
    // how are you gonna validate state of the current repository?
    // todo: check gix-oxide

    todo!("implement status which shows blob/tree contents");
}
