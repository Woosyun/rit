mod test_utils;

use rit::commands::*;
use tempdir::TempDir;

#[test]
pub fn work1() {
    use std::path::Path;

    let tempdir = TempDir::new("add_file_and_directory").unwrap();
    let cwd = tempdir.path().to_owned();

    let init = Init::build(cwd.clone()).unwrap();
    init.execute().expect("init failed");

    test_utils::work1(cwd.clone()).unwrap();

    let mut status1 = Status::build(cwd.clone()).expect("cannot build status");
    let mut status1_right = status1.clone();
    status1.execute().expect("cannot run status");
    status1_right.added.insert(Path::new("hello.txt").to_path_buf());
    status1_right.added.insert(Path::new("hello").to_path_buf());
    assert_eq!(status1_right, status1);

    let commit = Commit::build(cwd.clone()).expect("cannot build command");
    commit.execute("first commit".to_string()).expect("commit failed");

    let mut status2 = Status::build(cwd).expect("cannot build command");
    let status2_right = status2.clone();
    status2.execute().expect("cannot run status");
    assert_eq!(status2_right, status2);
}

#[test]
pub fn work2() {
    use std::path::Path;

    let tempdir = TempDir::new("delete_file_and_directory").unwrap();
    let cwd = tempdir.path().to_owned();

    let init = Init::build(cwd.clone()).unwrap();
    init.execute().unwrap();

    test_utils::work1(cwd.clone()).unwrap();

    let commit = Commit::build(cwd.clone()).unwrap();
    commit.execute("add file and directory".into()).unwrap();

    test_utils::work2(cwd.clone()).unwrap();

    let mut status = Status::build(cwd.clone()).unwrap();
    let mut test_status = status.clone();
    status.execute().unwrap();

    test_status.added.insert(Path::new("new.txt").to_path_buf());
    test_status.modified.insert(Path::new("hello.txt").to_path_buf());
    test_status.deleted.insert(Path::new("hello/world.txt").to_path_buf());

    assert_eq!(test_status, status);
}
