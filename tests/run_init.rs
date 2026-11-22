mod common;

use common::TestHarness;
use rit::prelude::*;
use std::fs;
use tempdir::TempDir;

#[test]
fn init_succeeds_on_empty_directory() {
    let th = TestHarness::new();
    th.init();

    let repo = th.rit_path().join(REPOSITORY);
    assert!(repo.is_dir(), ".git directory should exist");
    assert!(repo.join(DATABASE).is_dir(), ".git/objects should exist");
    assert!(
        repo.join(REFS).join(HEADS).is_dir(),
        ".git/refs/heads should exist"
    );
    assert!(repo.join(HEAD).is_file(), ".git/HEAD should exist");
    assert!(
        !repo.join(INDEX).exists(),
        ".git/index should not exist immediately after init"
    );

    let head = fs::read_to_string(repo.join(HEAD)).expect("HEAD should be readable");
    assert!(
        head.starts_with("ref: refs/heads/") && head.ends_with('\n'),
        "HEAD should point to an unborn branch symref ending in newline, got: {:?}",
        head
    );
}

#[test]
fn init_is_idempotent_on_existing_empty_repository() {
    let th = TestHarness::new();
    th.init();
    th.reinit();
    th.reinit();
}

#[test]
fn init_preserves_existing_commits_and_objects_on_reinit() {
    let th = TestHarness::new();
    th.init();
    th.write_file("hello.txt", "world\n");
    th.commit("initial commit", "Alice", "alice@example.com");

    let head_before = th
        .head_oid_rit()
        .expect("HEAD should resolve before reinit");
    let objects_before =
        common::read_objects(&th.rit_path().join(REPOSITORY)).expect("objects before reinit");

    // Re-initializing an existing repository must never wipe history, refs, or index.
    th.reinit();

    let head_after = th.head_oid_rit().expect("HEAD should resolve after reinit");
    let objects_after =
        common::read_objects(&th.rit_path().join(REPOSITORY)).expect("objects after reinit");

    assert_eq!(head_before, head_after);
    assert_eq!(objects_before, objects_after);
}

#[test]
fn init_preserves_custom_branch_head_on_reinit() {
    let th = TestHarness::new();
    th.init();
    th.set_head_branch("develop");
    th.write_file("file.txt", "content");
    th.commit("commit on develop", "Alice", "alice@example.com");

    th.reinit();

    let head = fs::read_to_string(th.rit_path().join(REPOSITORY).join(HEAD))
        .expect("HEAD should still exist");
    assert_eq!(head, "ref: refs/heads/develop\n");
}

#[test]
fn init_preserves_detached_head_on_reinit() {
    let th = TestHarness::new();
    th.init();
    th.write_file("a.txt", "a");
    th.commit("first", "Alice", "alice@example.com");

    let detached_oid = th.detach_head_to_current();
    th.reinit();

    let head = fs::read_to_string(th.rit_path().join(REPOSITORY).join(HEAD))
        .expect("HEAD should still exist");
    assert_eq!(head, format!("{detached_oid}\n"));
}

#[test]
fn init_on_directory_with_preexisting_files_leaves_files_untouched() {
    let th = TestHarness::new();
    th.write_file("README.md", "# Project\n");
    th.write_file("src/main.rs", "fn main() {}\n");

    th.init();

    assert_eq!(
        fs::read_to_string(th.rit_path().join("README.md")).unwrap(),
        "# Project\n"
    );
    assert_eq!(
        fs::read_to_string(th.rit_path().join("src/main.rs")).unwrap(),
        "fn main() {}\n"
    );
    assert!(
        !th.rit_path().join(REPOSITORY).join(INDEX).exists(),
        "init must not automatically stage preexisting files"
    );
}

#[test]
fn init_builder_fails_when_working_directory_not_set() {
    let result = init::Builder::new().build();
    assert!(
        result.is_err(),
        "init::Builder::build should fail when working_directory is omitted"
    );
}

#[test]
fn init_fails_when_working_directory_does_not_exist() {
    let temp = TempDir::new("rit-init-missing").unwrap();
    let nonexistent = temp.path().join("does-not-exist");

    let res = init::Builder::new()
        .working_directory(&nonexistent)
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .and_then(|cmd| cmd.execute());

    assert!(
        res.is_err(),
        "init should fail when target working directory does not exist"
    );
}

#[test]
fn init_fails_when_working_directory_is_a_regular_file() {
    let temp = TempDir::new("rit-init-file").unwrap();
    let file_path = temp.path().join("regular_file");
    fs::write(&file_path, "not a directory").unwrap();

    let res = init::Builder::new()
        .working_directory(&file_path)
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .and_then(|cmd| cmd.execute());

    assert!(
        res.is_err(),
        "init should fail when working directory path is a regular file"
    );
}

#[test]
fn init_fails_when_dot_git_exists_as_a_regular_file() {
    let temp = TempDir::new("rit-init-dotgit-file").unwrap();
    let dot_git = temp.path().join(REPOSITORY);
    fs::write(&dot_git, "not a directory").unwrap();

    let res = init::Builder::new()
        .working_directory(temp.path())
        .build()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .and_then(|cmd| cmd.execute());

    assert!(
        res.is_err(),
        "init should fail when .git already exists as a regular file instead of a directory"
    );
}
