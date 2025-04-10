use rit::{
    prelude::*,
    commands::*,
};
use tempdir::TempDir;
use std::{
    path::Path,
    fs,
    io,
};

fn work(workdir: &Path) -> io::Result<()> {
    let mut temp_file1 = workdir.to_path_buf();
    temp_file1.push("hello.txt");
    fs::write(&temp_file1, "hello").unwrap();

    let mut temp_file2 = workdir.to_path_buf();
    temp_file2.push("hello");
    fs::create_dir(&temp_file2).unwrap();
    temp_file2.push("world.txt");
    fs::write(&temp_file2, "world").unwrap();

    Ok(())
}

#[test]
pub fn commit() {
    let tempdir = TempDir::new("commit_once.commit").unwrap();
    let workdir = tempdir.path();

    let init = Init::build(workdir.to_path_buf()).unwrap();
    init.execute().unwrap();

    work(workdir).unwrap();

    let commit = Commit::build(workdir.to_path_buf())
        .expect("cannot build commit command");
    commit.execute("first commit".into())
        .expect("commit failed");

    let ws = Workspace::build(workdir.to_path_buf()).unwrap();
    let repo = Repository::build(&ws).unwrap();

    let head = repo.get_head()
        .expect("something is wrong and could not get head")
        .expect("head not updated after commit");
    let commit: repository::Commit = repo.db.retrieve(&head)
        .expect("cannot get commit objects");
    assert_eq!(commit.message(), "first commit");
    assert_eq!(commit.parent(), &None::<Oid>);
    //assert_eq!(&commit.root, root);
}

#[test]
pub fn status() {
    let tempdir = TempDir::new("commit_once.status").unwrap();
    let workdir = tempdir.path();

    let init = Init::build(workdir.to_path_buf()).unwrap();
    init.execute().unwrap();

    work(workdir).unwrap();

    let status = Status::build(workdir.to_path_buf())
        .expect("cannot build status command");
    let rev_diff = status.scan()
        .expect("failed to scan");
    let mut pc_rev_diff = RevDiff::new();
    let _ = pc_rev_diff.added.insert(Path::new("hello.txt").to_path_buf());
    let _ = pc_rev_diff.added.insert(Path::new("hello/world.txt").to_path_buf());
    assert_eq!(rev_diff, pc_rev_diff);

    let commit = Commit::build(workdir.to_path_buf()).unwrap();
    commit.execute("first commit".into()).unwrap();

    let status = Status::build(workdir.to_path_buf())
        .expect("cannot build status command");
    let rev_diff = status.scan()
        .expect("failed to scan");
    let pc_rev_diff = RevDiff::new();
    assert_eq!(rev_diff, pc_rev_diff);
}
