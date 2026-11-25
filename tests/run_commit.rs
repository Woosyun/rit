mod common;

use common::TestHarness;
use rit::prelude::*;
use std::fs;
use tempdir::TempDir;

// =============================================================================
// Builder & Precondition Error Cases
// =============================================================================

#[test]
fn commit_fails_without_repository_initialized() {
    // Empty uninitialized directory
    let th_empty = TestHarness::new();
    let res_empty = th_empty.try_rit_commit(
        "erroneous commit",
        "me",
        "me@example.com",
        "me",
        "me@example.com",
    );
    assert!(
        res_empty.is_err(),
        "commit must return Err on an empty uninitialized directory"
    );
    assert!(
        !th_empty.rit_path().join(REPOSITORY).exists(),
        "failed commit must not create a .git directory"
    );

    // Uninitialized directory with files
    let th = TestHarness::new();
    th.write_file("a.txt", "hello\n");

    let res = th.try_rit_commit(
        "erroneous commit",
        "me",
        "me@example.com",
        "me",
        "me@example.com",
    );
    assert!(
        res.is_err(),
        "commit must return Err when repository is not initialized"
    );
    assert!(
        !th.rit_path().join(REPOSITORY).exists(),
        "failed commit must not create a .git directory"
    );
}

#[test]
fn commit_builder_fails_when_working_directory_missing() {
    let res = commit::Builder::new()
        .message("msg")
        .author("me", "me@example.com")
        .committer("me", "me@example.com")
        .build();
    assert!(res.is_err(), "build should fail without working_directory");
}

#[test]
fn commit_builder_fails_when_message_missing() {
    let temp = TempDir::new("rit-commit-no-msg").unwrap();
    let res = commit::Builder::new()
        .working_directory(temp.path())
        .author("me", "me@example.com")
        .committer("me", "me@example.com")
        .build();
    assert!(res.is_err(), "build should fail without commit message");
}

#[test]
fn commit_builder_fails_when_author_missing() {
    let temp = TempDir::new("rit-commit-no-author").unwrap();
    let res = commit::Builder::new()
        .working_directory(temp.path())
        .message("msg")
        .committer("me", "me@example.com")
        .build();
    assert!(res.is_err(), "build should fail without author");
}

#[test]
fn commit_builder_fails_when_committer_missing() {
    let temp = TempDir::new("rit-commit-no-committer").unwrap();
    let res = commit::Builder::new()
        .working_directory(temp.path())
        .message("msg")
        .author("me", "me@example.com")
        .build();
    assert!(res.is_err(), "build should fail without committer");
}

#[test]
fn commit_fails_when_working_directory_does_not_exist() {
    let temp = TempDir::new("rit-commit-missing-dir").unwrap();
    let nonexistent = temp.path().join("nope");

    let res = commit::Builder::new()
        .working_directory(&nonexistent)
        .message("msg")
        .author("me", "me@example.com")
        .committer("me", "me@example.com")
        .build();
    assert!(res.is_err(), "build should fail for nonexistent directory");
}

// =============================================================================
// Basic & Multi-Commit History
// =============================================================================

#[test]
fn empty_repository_sequential_commits() {
    let th = TestHarness::new();
    th.init();

    // First (root) commit on an empty working tree: no `parent` header, empty tree OID.
    th.commit("first commit", "me", "me1234@example.com");
    let first_oid = th.head_oid_rit().unwrap();

    // Second & third commits build a linear parent chain.
    th.commit("second commit", "me", "me1234@example.com");
    let second_oid = th.head_oid_rit().unwrap();
    assert_ne!(first_oid, second_oid);

    th.commit("third commit", "me", "me1234@example.com");
    let third_oid = th.head_oid_rit().unwrap();
    assert_ne!(second_oid, third_oid);

    // Empty tree object (4b825dc642cb6eb9a060e54bf8d69288fbee4904) must exist in objects.
    let empty_tree_oid = "4b825dc642cb6eb9a060e54bf8d69288fbee4904"
        .try_into()
        .unwrap();
    let objects = common::read_objects(&th.rit_path().join(REPOSITORY)).unwrap();
    assert!(
        objects.contains_key(&empty_tree_oid),
        "Empty tree object 4b825dc642cb6eb9a060e54bf8d69288fbee4904 should be stored"
    );
}

#[test]
fn commit_single_file() {
    let th = TestHarness::new();
    th.init();
    th.write_file("hello.txt", "hello world\n");
    th.commit("add hello.txt", "Alice", "alice@example.com");
}

#[test]
fn commit_add_modify_delete_and_delete_all_lifecycle() {
    let th = TestHarness::new();
    th.init();

    // Commit 1: initial files
    th.write_file("a.txt", "alpha v1\n");
    th.write_file("b.txt", "beta v1\n");
    th.commit("c1: add a and b", "Alice", "alice@example.com");
    let objects_after_c1 = common::read_objects(&th.rit_path().join(REPOSITORY)).unwrap();

    // Commit 2: modify a.txt, add c.txt
    th.write_file("a.txt", "alpha v2\n");
    th.write_file("c.txt", "gamma v1\n");
    th.commit("c2: edit a and add c", "Alice", "alice@example.com");

    // Commit 3: delete b.txt, modify c.txt
    th.remove_file("b.txt");
    th.write_file("c.txt", "gamma v2\n");
    th.commit("c3: delete b and edit c", "Alice", "alice@example.com");

    // Commit 4: delete all remaining files (back to empty tree)
    th.remove_file("a.txt");
    th.remove_file("c.txt");
    th.commit("c4: remove everything", "Alice", "alice@example.com");

    // All historical objects from c1 must remain intact in the object database.
    let objects_after_c4 = common::read_objects(&th.rit_path().join(REPOSITORY)).unwrap();
    for (oid, raw) in &objects_after_c1 {
        assert_eq!(
            objects_after_c4.get(oid),
            Some(raw),
            "Historical object {} must not be lost or altered",
            // encode_oid_to_hex(oid)
            // <String as From<&ObjectId>>::from(oid)
            oid
        );
    }
}

#[test]
fn commit_deduplicates_identical_blobs_within_and_across_commits() {
    let th = TestHarness::new();
    th.init();

    // Three files in different directories with identical content must share one blob object.
    th.write_file("copy1.txt", "identical payload\n");
    th.write_file("copy2.txt", "identical payload\n");
    th.write_file("nested/copy3.txt", "identical payload\n");
    th.commit("add identical files", "Alice", "alice@example.com");

    // Committing again with no file changes must reuse existing blob and tree objects cleanly.
    th.commit("no-op commit", "Alice", "alice@example.com");

    // Modify one file, commit, then revert it back to original content.
    th.write_file("copy1.txt", "temporarily modified\n");
    th.commit("modify copy1", "Alice", "alice@example.com");

    th.write_file("copy1.txt", "identical payload\n");
    th.commit("revert copy1", "Alice", "alice@example.com");
}

// =============================================================================
// Author, Committer & Message Edge Cases
// =============================================================================

#[test]
fn commit_with_distinct_author_and_committer() {
    let th = TestHarness::new();
    th.init();
    th.write_file("patch.txt", "submitted by author, applied by committer\n");

    th.commit_with_committer(
        "apply external patch",
        "Original Author",
        "author@example.org",
        "Repo Maintainer",
        "maintainer@example.com",
    );
}

#[test]
fn commit_message_formatting_and_unicode_edge_cases() {
    let th = TestHarness::new();
    th.init();

    // Message without trailing newline (Git ensures a single trailing '\n')
    th.write_file("f.txt", "1");
    th.commit("no trailing newline", "Alice", "alice@example.com");

    // Message already ending with trailing newline (must not double-append '\n')
    th.write_file("f.txt", "2");
    th.commit("with trailing newline\n", "Alice", "alice@example.com");

    // Multiline message with blank line separating subject and body
    th.write_file("f.txt", "3");
    th.commit(
        "Subject line\n\nDetailed body paragraph 1.\n\n- bullet 1\n- bullet 2\n",
        "Alice",
        "alice@example.com",
    );

    // Unicode / CJK / emoji message and author/committer names
    th.write_file("f.txt", "4");
    th.commit_with_committer(
        "한글 커밋 메시지 🚀 café\n\n본문 내용",
        "홍길동",
        "hong@example.kr",
        "José García",
        "jose@example.es",
    );

    // Empty commit message
    th.write_file("f.txt", "5");
    th.commit("", "Alice", "alice@example.com");
}

// =============================================================================
// References, Detached HEAD & Symbolic Ref Chains
// =============================================================================

#[test]
fn commit_on_detached_head_updates_head_directly_without_moving_branch() {
    let th = TestHarness::new();
    th.init();

    th.write_file("v.txt", "v1\n");
    th.commit("c1 on branch", "Alice", "alice@example.com");

    let refs_before = common::read_references(&th.rit_path().join(REPOSITORY)).unwrap();
    let detached_base = th.detach_head_to_current();

    th.write_file("v.txt", "v2\n");
    th.commit("c2 on detached HEAD", "Alice", "alice@example.com");

    let new_head = fs::read_to_string(th.rit_path().join(REPOSITORY).join(HEAD)).unwrap();
    assert_ne!(
        new_head.trim(),
        detached_base,
        "Detached HEAD should advance to new commit"
    );

    let refs_after = common::read_references(&th.rit_path().join(REPOSITORY)).unwrap();
    assert_eq!(
        refs_before, refs_after,
        "Committing on detached HEAD must not modify branch refs under refs/heads"
    );
}

#[test]
fn commit_on_nested_branch_name_creates_ref_parent_directories() {
    let th = TestHarness::new();
    th.init();

    // Point HEAD to an unborn branch inside nested directories: refs/heads/feature/deep/topic
    th.set_head_branch("feature/deep/topic");

    th.write_file("feature.txt", "new feature\n");
    th.commit("commit on nested branch", "Alice", "alice@example.com");

    let ref_file = th
        .rit_path()
        .join(REPOSITORY)
        .join(REFS)
        .join(HEADS)
        .join("feature/deep/topic");
    assert!(
        ref_file.is_file(),
        "Nested branch ref file {:?} should be created",
        ref_file
    );

    // Second commit on the same nested branch updates the ref properly
    th.write_file("feature.txt", "new feature v2\n");
    th.commit(
        "second commit on nested branch",
        "Alice",
        "alice@example.com",
    );
}

#[test]
fn commit_through_chained_symbolic_refs_updates_target_ref() {
    let th = TestHarness::new();
    th.init();

    th.write_file("base.txt", "base\n");
    th.commit("base commit", "Alice", "alice@example.com");

    // Discover the initial branch name (e.g. main or master) and create:
    // HEAD -> ref: refs/heads/alias -> ref: refs/heads/<initial_branch>
    let head_content = fs::read_to_string(th.rit_path().join(REPOSITORY).join(HEAD)).unwrap();
    let initial_target = head_content
        .strip_prefix("ref: ")
        .unwrap()
        .trim_end_matches('\n')
        .to_string();

    th.write_symref("refs/heads/alias", &initial_target);
    th.set_head_branch("alias");

    th.write_file("base.txt", "updated through symref chain\n");
    th.commit("commit via chained symref", "Alice", "alice@example.com");

    // Both HEAD and refs/heads/alias must remain symbolic refs; only the final target ref moves.
    let alias_raw =
        fs::read_to_string(th.rit_path().join(REPOSITORY).join("refs/heads/alias")).unwrap();
    assert_eq!(alias_raw, format!("ref: {initial_target}\n"));
}

#[test]
fn commit_fails_when_index_lock_is_held() {
    let th = TestHarness::new();
    th.init();
    th.write_file("a.txt", "v1\n");
    th.commit("initial", "Alice", "alice@example.com");

    let index_path = th.rit_path().join(REPOSITORY).join(INDEX);
    let index_before = fs::read(&index_path).unwrap();
    let head_before = th.head_oid_rit().unwrap();

    // Hold `.git/index.lock` externally to simulate a concurrent writer.
    let mut lock = LockFile::new(&index_path);
    lock.hold().expect("Should acquire index.lock");

    th.write_file("a.txt", "v2\n");
    let res = th.try_rit_commit(
        "should fail due to lock",
        "Alice",
        "alice@example.com",
        "Alice",
        "alice@example.com",
    );
    assert!(
        res.is_err(),
        "commit should fail cleanly when .git/index.lock is already held"
    );

    drop(lock);

    // Original index and HEAD must remain untouched.
    assert_eq!(fs::read(&index_path).unwrap(), index_before);
    assert_eq!(th.head_oid_rit().unwrap(), head_before);
}
