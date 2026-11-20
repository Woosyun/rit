mod common;

use common::TestHarness;
use rit::prelude::*;

#[test]
fn nested_directories_and_deep_hierarchy() {
    let th = TestHarness::new();
    th.init();

    th.write_file("root.txt", "root\n");
    th.write_file("a/1.txt", "level 1\n");
    th.write_file("a/b/2.txt", "level 2\n");
    th.write_file("a/b/c/d/e/leaf.txt", "deep leaf\n");
    th.write_file("a/sibling/s.txt", "sibling\n");
    th.write_file("x/y/z.txt", "another branch\n");

    th.commit("deep directory tree", "Alice", "alice@example.com");

    // Modify one deeply nested leaf; unchanged subtrees (e.g. `x/y`) should still match git.
    th.write_file("a/b/c/d/e/leaf.txt", "updated deep leaf\n");
    th.commit("update deep leaf", "Alice", "alice@example.com");
}

#[test]
fn empty_directories_are_ignored_and_pruned_when_emptied() {
    let th = TestHarness::new();
    th.init();

    // Empty directories must never appear in Git trees or index.
    th.create_dir("empty_top");
    th.create_dir("nested/empty/deep");
    th.write_file("tracked/keep.txt", "keep\n");
    th.write_file("will_empty/temp.txt", "temp\n");

    th.commit("commit with empty dirs present", "Alice", "alice@example.com");

    // Delete the only file in `will_empty/`, leaving `will_empty/` as an empty directory.
    // The `will_empty` tree entry must disappear from the root tree.
    th.remove_file("will_empty/temp.txt");
    th.commit("prune emptied directory", "Alice", "alice@example.com");
}

#[test]
fn git_tree_sorting_order_files_vs_directories() {
    // Classic Git edge case:
    // In Git tree objects, directory entries are sorted as if they have a trailing '/'.
    // ASCII order: '-' (45) < '.' (46) < '/' (47) < '0' (48) < '_' (95)
    // Correct order: "foo-bar" < "foo.txt" < "foo/" < "foo0" < "foo_bar"
    // Naive sorting without trailing '/' puts "foo" first and fails `git fsck` (`treeNotSorted`).
    let th = TestHarness::new();
    th.init();

    th.write_file("foo_bar", "underscore\n");
    th.write_file("foo0", "digit\n");
    th.write_file("foo/inside.txt", "dir child\n");
    th.write_file("foo.txt", "dot\n");
    th.write_file("foo-bar", "hyphen\n");

    // Also test the exact same tricky ordering inside a subdirectory!
    th.write_file("sub/a_b", "1");
    th.write_file("sub/a0", "2");
    th.write_file("sub/a/child", "3");
    th.write_file("sub/a.b", "4");
    th.write_file("sub/a-b", "5");

    th.commit("verify canonical git tree sorting", "Alice", "alice@example.com");
}

#[test]
fn file_mode_normalization_regular_and_executable() {
    let th = TestHarness::new();
    th.init();

    // All non-executable regular files normalize to 100644 in Git.
    th.write_file("m600.txt", "600");
    th.set_mode("m600.txt", 0o600);

    th.write_file("m644.txt", "644");
    th.set_mode("m644.txt", 0o644);

    th.write_file("m664.txt", "664");
    th.set_mode("m664.txt", 0o664);

    th.write_file("m444.txt", "444");
    th.set_mode("m444.txt", 0o444);

    // All user-executable regular files normalize to 100755 in Git.
    th.write_file("m700.sh", "#!/bin/sh\necho 700\n");
    th.set_mode("m700.sh", 0o700);

    th.write_file("m711.sh", "#!/bin/sh\necho 711\n");
    th.set_mode("m711.sh", 0o711);

    th.write_executable("bin/run.sh", "#!/bin/sh\necho 755\n");

    th.commit("file modes normalized", "Alice", "alice@example.com");

    // Restore write bit on 0o444 file for clean teardown across platforms.
    th.set_mode("m444.txt", 0o644);
}

#[test]
fn mode_only_change_updates_tree_and_index_reusing_blob() {
    let th = TestHarness::new();
    th.init();

    th.write_file("script.sh", "#!/bin/sh\necho hi\n");
    th.set_mode("script.sh", 0o644);
    th.commit("add non-executable script", "Alice", "alice@example.com");

    // Flip only the executable bit without modifying file bytes.
    th.set_mode("script.sh", 0o755);
    th.commit("make script executable", "Alice", "alice@example.com");

    // Flip back to 0o644.
    th.set_mode("script.sh", 0o644);
    th.commit("remove executable bit", "Alice", "alice@example.com");
}

#[test]
fn file_replaced_by_directory_and_directory_replaced_by_file() {
    // Classic VCS D/F (directory/file) transition across commits.
    let th = TestHarness::new();
    th.init();

    // Commit 1: `entry` is a regular file.
    th.write_file("entry", "i am a regular file\n");
    th.write_file("other.txt", "unchanged\n");
    th.commit("entry is a file", "Alice", "alice@example.com");

    // Commit 2: replace file `entry` with directory `entry/child.txt`.
    th.remove_file("entry");
    th.write_file("entry/child.txt", "now inside a directory\n");
    th.commit("entry becomes a directory", "Alice", "alice@example.com");

    // Commit 3: replace directory `entry/` back with a regular file `entry`.
    th.remove_dir_all("entry");
    th.write_file("entry", "regular file again\n");
    th.commit("entry becomes a file again", "Alice", "alice@example.com");
}

#[test]
fn empty_binary_and_large_files() {
    let th = TestHarness::new();
    th.init();

    // 1. 0-byte empty file -> well-known empty blob OID e69de29bb2d1d6434b8b29ae777ad8c2e48c5391
    th.write_file("empty.bin", b"");

    // 2. Binary file containing every byte 0x00..=0xFF, NUL bytes, and invalid UTF-8
    let mut all_bytes = Vec::with_capacity(512);
    for b in 0u8..=255 {
        all_bytes.push(b);
    }
    all_bytes.extend_from_slice(&[0x00, 0xff, 0xfe, 0x00, b'b', b'l', b'o', b'b', 0x00]);
    th.write_file("binary.dat", &all_bytes);

    // 3. Large deterministic file (~512 KiB) to exercise multi-chunk hashing and zlib compression
    let large: Vec<u8> = (0..524_288u32)
        .map(|i| ((i.wrapping_mul(31).wrapping_add(17)) & 0xff) as u8)
        .collect();
    th.write_file("large.bin", &large);

    th.commit("empty, binary, and large blobs", "Alice", "alice@example.com");

    let empty_blob_oid = decode_str_to_oid("e69de29bb2d1d6434b8b29ae777ad8c2e48c5391").unwrap();
    let objects = common::read_objects(&th.rit_path().join(REPOSITORY)).unwrap();
    assert_eq!(
        objects.get(&empty_blob_oid).map(Vec::as_slice),
        Some(b"blob 0\0".as_slice()),
        "Empty blob object must match canonical Git OID and payload"
    );
}

#[test]
fn special_hidden_unicode_and_long_filenames() {
    let th = TestHarness::new();
    th.init();

    th.write_file("hello world.txt", "spaces in root name\n");
    th.write_file("dir with spaces/my file.txt", "spaces in dir and file\n");
    th.write_file(".env", "SECRET=1\n");
    th.write_file(".hidden_dir/.secret", "hidden\n");
    th.write_file("..double_dot", "two leading dots\n");
    th.write_file("한글파일.txt", "korean filename\n");
    th.write_file("café/naïve_🚀.txt", "unicode path\n");
    th.write_file("a+b=c,d;e@f#g.txt", "punctuation\n");

    let long_name = format!("long_dir/{}.txt", "x".repeat(120));
    th.write_file(&long_name, "long filename content\n");

    th.commit("special and unicode filenames", "Alice", "alice@example.com");
}

#[test]
fn dot_git_directory_is_never_committed_but_dot_git_prefixed_files_are() {
    let th = TestHarness::new();
    th.init();

    // Files whose names merely START with ".git" MUST still be tracked;
    // only the repository metadata directory `.git` itself must be ignored.
    th.write_file(".gitkeep", "");
    th.write_file(".gitattributes", "* text=auto\n");
    th.write_file(".github/workflows/ci.yml", "name: CI\n");
    th.write_file("src/app.txt", "app\n");

    th.commit("first commit", "Alice", "alice@example.com");

    // Second commit after `.git/index`, `.git/objects/*`, `.git/refs/heads/*` all exist:
    // none of `.git/**` may leak into the index or tree.
    th.write_file("src/app.txt", "app v2\n");
    th.commit("second commit", "Alice", "alice@example.com");

    let index = common::read_index(&th.rit_path().join(REPOSITORY)).unwrap();
    for entry in index.entries() {
        let first_component = entry
            .relative_path()
            .components()
            .next()
            .unwrap()
            .as_os_str()
            .to_string_lossy();
        assert_ne!(
            first_component, ".git",
            "Internal .git entry leaked into index: {:?}",
            entry.relative_path()
        );
    }
}

#[test]
fn rename_and_move_files_across_directories() {
    let th = TestHarness::new();
    th.init();

    th.write_file("old.txt", "moved content\n");
    th.write_file("a/b/deep.txt", "deep moved content\n");
    th.commit("before rename", "Alice", "alice@example.com");

    th.rename("old.txt", "new.txt");
    th.rename("a/b/deep.txt", "c/d/deep.txt");
    th.commit("after rename and cross-dir move", "Alice", "alice@example.com");
}

#[test]
fn index_entry_padding_covers_all_mod_8_path_lengths() {
    // In Git index v2, the fixed fields before the path take 62 bytes (62 % 8 == 6).
    // Path lengths 1..=16 cover every residue class mod 8, including the classic off-by-one
    // boundary at len == 2 and len == 10 where (62 + len) % 8 == 0 and 8 NUL bytes must be
    // written (1 NUL terminator + 7 padding NUL bytes).
    let th = TestHarness::new();
    th.init();

    for len in 1..=16 {
        let name = "a".repeat(len);
        th.write_file(&name, format!("len={len}\n"));
    }

    th.commit("all mod-8 index entry lengths", "Alice", "alice@example.com");
}

#[test]
fn many_files_with_ascii_case_and_symbol_sorting() {
    // Verifies unsigned-byte lexicographical sorting in both index and trees:
    // '0' (48) < 'A' (65) < 'Z' (90) < '_' (95) < 'a' (97) < 'z' (122).
    let th = TestHarness::new();
    th.init();

    let prefixes = ["0", "9", "A", "M", "Z", "_", "a", "m", "z"];
    for (i, p) in prefixes.iter().enumerate() {
        for j in 0..8 {
            let name = format!("{p}_{j:02}.txt");
            th.write_file(&name, format!("item {i}-{j}\n"));
        }
    }
    th.write_file("DirA/file.txt", "upper dir\n");
    th.write_file("dira/file.txt", "lower dir\n");

    th.commit("many files sorted", "Alice", "alice@example.com");
}
