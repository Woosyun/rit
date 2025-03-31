use rit::commands::{Init, Commit};
use tempdir::TempDir;
mod works;

#[test]
pub fn initialize_repository() {
    let tempdir = TempDir::new("init").unwrap();
    let cwd = tempdir.path();
    
    let init = Init::build(cwd.to_path_buf()).unwrap();
    let _ = init.execute().unwrap();
}

#[test]
pub fn commit_once() {
    let tempdir = TempDir::new("commit-once").unwrap();
    let cwd = tempdir.path();

    let init = Init::build(cwd.to_path_buf()).expect("cannot build command");
    init.execute().expect("init failed");

    works::work1::run(cwd.to_path_buf()).unwrap();

    let commit = Commit::build(cwd.to_path_buf()).expect("cannot build command");
    commit.execute("first commit".to_string()).expect("commit failed");
}
