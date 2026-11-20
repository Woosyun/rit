mod common;

use common::{git_commit_env, run_git_checked};
use rit::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tempdir::TempDir;

#[test]
fn git_reads_full_history_and_trees_created_by_rit() {
    let dir = TempDir::new("rit-interop-git-reads-rit").unwrap();
    let root = dir.path();

    init::Builder::new()
        .working_directory(root)
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // Commit 1
    fs::create_dir_all(root.join("src")).unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn v() -> u32 { 1 }\n").unwrap();
    fs::write(root.join("obsolete.txt"), "will be removed\n").unwrap();
    commit::Builder::new()
        .working_directory(root)
        .message("c1: initial layout")
        .author("Alice", "alice@example.com")
        .committer("Alice", "alice@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // Commit 2: add executable, update src/lib.rs
    fs::write(root.join("src/lib.rs"), "pub fn v() -> u32 { 2 }\n").unwrap();
    let script = root.join("run.sh");
    fs::write(&script, "#!/bin/sh\necho ok\n").unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    commit::Builder::new()
        .working_directory(root)
        .message("c2: add script and bump version")
        .author("Bob", "bob@example.com")
        .committer("Bob", "bob@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // Commit 3: delete obsolete.txt
    fs::remove_file(root.join("obsolete.txt")).unwrap();
    commit::Builder::new()
        .working_directory(root)
        .message("c3: remove obsolete file")
        .author("Alice", "alice@example.com")
        .committer("Alice", "alice@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // Real git must accept the repository completely cleanly.
    run_git_checked(root, &["fsck", "--full", "--strict"], &[]);

    let log = run_git_checked(root, &["log", "--format=%s"], &[]);
    assert_eq!(
        String::from_utf8_lossy(&log.stdout),
        "c3: remove obsolete file\nc2: add script and bump version\nc1: initial layout\n"
    );

    let show_current = run_git_checked(root, &["show", "HEAD:src/lib.rs"], &[]);
    assert_eq!(
        String::from_utf8_lossy(&show_current.stdout),
        "pub fn v() -> u32 { 2 }\n"
    );

    let show_old = run_git_checked(root, &["show", "HEAD~2:obsolete.txt"], &[]);
    assert_eq!(
        String::from_utf8_lossy(&show_old.stdout),
        "will be removed\n"
    );

    let ls_tree = run_git_checked(root, &["ls-tree", "-r", "HEAD"], &[]);
    let ls_tree_str = String::from_utf8_lossy(&ls_tree.stdout);
    assert!(ls_tree_str.contains("100755 blob "), "run.sh should have mode 100755");
    assert!(ls_tree_str.contains("\trun.sh\n"));
    assert!(ls_tree_str.contains("\tsrc/lib.rs\n"));
    assert!(!ls_tree_str.contains("obsolete.txt"));

    let status = run_git_checked(root, &["status", "--porcelain"], &[]);
    assert_eq!(
        String::from_utf8_lossy(&status.stdout),
        "",
        "git status should report a clean working tree right after rit commit"
    );
}

#[test]
fn rit_commits_on_top_of_git_created_repository() {
    let dir = TempDir::new("rit-interop-rit-on-git").unwrap();
    let root = dir.path();
    let env = git_commit_env("Alice", "alice@example.com", "Alice", "alice@example.com");

    run_git_checked(root, &["init", "-b", "main"], &env);
    fs::write(root.join("README.md"), "# Hello\n").unwrap();
    run_git_checked(root, &["add", "-A"], &env);
    run_git_checked(root, &["commit", "-m", "c1 from git"], &env);

    let c1_oid = String::from_utf8_lossy(
        &run_git_checked(root, &["rev-parse", "HEAD"], &env).stdout,
    )
    .trim()
    .to_string();

    // Now commit using rit on top of the git-initialized repo.
    fs::write(root.join("README.md"), "# Hello from rit\n").unwrap();
    fs::write(root.join("added_by_rit.txt"), "works!\n").unwrap();

    commit::Builder::new()
        .working_directory(root)
        .message("c2 from rit")
        .author("Bob", "bob@example.com")
        .committer("Bob", "bob@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    run_git_checked(root, &["fsck", "--full", "--strict"], &env);

    let parent_of_head = String::from_utf8_lossy(
        &run_git_checked(root, &["rev-parse", "HEAD~1"], &env).stdout,
    )
    .trim()
    .to_string();
    assert_eq!(
        parent_of_head, c1_oid,
        "rit commit must point to git's commit as its parent"
    );

    let log = run_git_checked(root, &["log", "--format=%s"], &env);
    assert_eq!(
        String::from_utf8_lossy(&log.stdout),
        "c2 from rit\nc1 from git\n"
    );

    let status = run_git_checked(root, &["status", "--porcelain"], &env);
    assert_eq!(String::from_utf8_lossy(&status.stdout), "");
}

#[test]
fn git_and_rit_interleaved_commits_produce_valid_history() {
    let dir = TempDir::new("rit-interop-interleaved").unwrap();
    let root = dir.path();
    let env = git_commit_env("Alice", "alice@example.com", "Alice", "alice@example.com");

    // 1. Init with rit + c1 with rit
    init::Builder::new()
        .working_directory(root)
        .build()
        .unwrap()
        .execute()
        .unwrap();
    fs::write(root.join("step.txt"), "1\n").unwrap();
    commit::Builder::new()
        .working_directory(root)
        .message("step 1 (rit)")
        .author("Alice", "alice@example.com")
        .committer("Alice", "alice@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // 2. c2 with git
    fs::write(root.join("step.txt"), "2\n").unwrap();
    run_git_checked(root, &["add", "-A"], &env);
    run_git_checked(root, &["commit", "-m", "step 2 (git)"], &env);

    // 3. c3 with rit
    fs::write(root.join("step.txt"), "3\n").unwrap();
    commit::Builder::new()
        .working_directory(root)
        .message("step 3 (rit)")
        .author("Alice", "alice@example.com")
        .committer("Alice", "alice@example.com")
        .build()
        .unwrap()
        .execute()
        .unwrap();

    // 4. c4 with git
    fs::write(root.join("step.txt"), "4\n").unwrap();
    run_git_checked(root, &["add", "-A"], &env);
    run_git_checked(root, &["commit", "-m", "step 4 (git)"], &env);

    run_git_checked(root, &["fsck", "--full", "--strict"], &env);

    let log = run_git_checked(root, &["log", "--format=%s"], &env);
    assert_eq!(
        String::from_utf8_lossy(&log.stdout),
        "step 4 (git)\nstep 3 (rit)\nstep 2 (git)\nstep 1 (rit)\n"
    );

    for (rev, expected) in [
        ("HEAD:step.txt", "4\n"),
        ("HEAD~1:step.txt", "3\n"),
        ("HEAD~2:step.txt", "2\n"),
        ("HEAD~3:step.txt", "1\n"),
    ] {
        let out = run_git_checked(root, &["show", rev], &env);
        assert_eq!(String::from_utf8_lossy(&out.stdout), expected);
    }

    let status = run_git_checked(root, &["status", "--porcelain"], &env);
    assert_eq!(String::from_utf8_lossy(&status.stdout), "");
}
