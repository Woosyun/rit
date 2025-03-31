mod works;

use rit::commands::*;
use tempdir::TempDir;
use std::path::Path;

#[test]
pub fn status() {
    let tempdir = TempDir::new("commit-once").unwrap();
    let cwd = tempdir.path();

    let init = Init::build(cwd.to_path_buf()).expect("cannot build command");
    init.execute().expect("init failed");

    works::work1::run(cwd.to_path_buf()).unwrap();

    //check status for work1. hello.txt & hello/world.txt
    let mut status1 = Status::build(cwd.to_path_buf()).expect("cannot build status");
    let mut status1_right = status1.clone();
    status1.execute().expect("cannot run status");
    status1_right.added.insert(Path::new("hello.txt").to_path_buf());
    status1_right.added.insert(Path::new("hello").to_path_buf());
    assert_eq!(status1_right, status1);

    let commit = Commit::build(cwd.to_path_buf()).expect("cannot build command");
    commit.execute("first commit".to_string()).expect("commit failed");

    let mut status2 = Status::build(cwd.to_path_buf()).expect("cannot build command");
    let status2_right = status2.clone();
    status2.execute().expect("cannot run status");
    assert_eq!(status2_right, status2);
}
