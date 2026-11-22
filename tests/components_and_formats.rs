mod common;

use common::run_git_checked;
use flate2::read::ZlibDecoder;
use rit::prelude::*;
use rit::repository::database::Database;
use std::assert_matches;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::sync::{Arc, Barrier};
use std::thread;
use tempdir::TempDir;

// =============================================================================
// OID Hex Codec
// =============================================================================

#[test]
fn oid_hex_encode_decode_roundtrip_and_invalid_inputs() {
    let hex = "0123456789abcdef0123456789abcdef01234567";
    let oid = ObjectId::try_from(hex).expect("valid lowercase 40-char hex should decode");
    assert_eq!(String::from(&oid), hex);

    // Uppercase hex should decode to the same bytes and re-encode lowercase.
    let upper = hex.to_ascii_uppercase();
    let oid_upper =
        ObjectId::try_from(upper.as_str()).expect("valid uppercase 40-char hex should decode");
    assert_eq!(oid_upper, oid);

    // Invalid lengths and characters must return Err (never panic).
    let invalid_cases = [
        "".to_string(),
        "0".repeat(39),
        "0".repeat(41),
        format!("{}g", "0".repeat(39)),
        format!("{} ", "0".repeat(39)),
        // 20 two-byte UTF-8 characters == 40 bytes, 20 chars: must return Err, not panic!
        "é".repeat(20),
        // U+0130 ('İ') has low byte 0x30 ('0') and 2 UTF-8 bytes (40 bytes total):
        // must return Err rather than truncating char->u8 and panicking on slice length!
        "\u{0130}".repeat(20),
    ];
    for bad in &invalid_cases {
        assert!(
            ObjectId::try_from(bad.as_str()).is_err(),
            "decode_str_to_oid should reject {:?}",
            bad
        );
    }
}

// =============================================================================
// Object & ObjectRaw Serialization / Parsing
// =============================================================================

#[test]
fn blob_object_raw_matches_git_format_and_roundtrips_binary_with_nulls() {
    // 1. Canonical empty blob
    let empty_obj: Object = Blob::new(vec![]).into();
    let empty_raw = ObjectRaw::from(empty_obj);
    assert_eq!(empty_raw.data, b"blob 0\0");
    assert_eq!(
        String::from(&hash(&empty_raw.data)),
        "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391",
    );

    // 2. Canonical "hello\n" blob
    let hello_obj: Object = Blob::new(b"hello\n".to_vec()).into();
    let hello_raw = ObjectRaw::from(hello_obj);
    assert_eq!(hello_raw.data, b"blob 6\0hello\n");
    assert_eq!(
        String::from(&hash(&hello_raw.data)),
        "ce013625030ba8dba906f756967f9e9ca394464a"
    );

    // 3. Binary blob containing embedded NUL bytes must roundtrip cleanly through ObjectRaw::parse
    let payload = b"ab\0cd\0ef".to_vec();
    let bin_obj: Object = Blob::new(payload.clone()).into();
    let bin_raw = ObjectRaw::from(bin_obj);
    assert_eq!(bin_raw.data, b"blob 8\0ab\0cd\0ef");

    //let parsed = bin_raw.parse().expect("ObjectRaw::parse should succeed");
    let parsed = Object::try_from(bin_raw).expect("Parsing should succeed on valid raw object");
    assert_matches!(parsed.kind, ObjectKind::Blob);
    assert_eq!(parsed.size, payload.len() as u32);
    assert_eq!(parsed.data, payload);
}

#[test]
fn object_raw_parse_rejects_malformed_objects() {
    let malformed: &[&[u8]] = &[
        b"",
        b"blob 5",             // missing NUL separator
        b"blob5\0hello",       // missing space in header
        b"unknown 5\0hello",   // unknown object type
        b"blob notanumber\0a", // non-numeric size
    ];

    for raw in malformed {
        let obj_raw = ObjectRaw::new(raw.to_vec());
        assert!(
            Object::try_from(obj_raw).is_err(),
            //obj_raw.parse().is_err(),
            "ObjectRaw::parse should reject malformed input {:?}",
            String::from_utf8_lossy(raw)
        );
    }
}

#[test]
fn tree_object_format_matches_git_octal_modes_and_sorting() {
    // 1. Empty tree -> well-known Git empty tree OID
    let empty_tree: Object = Tree::new(vec![]).into();
    let empty_raw = ObjectRaw::from(empty_tree);
    assert_eq!(empty_raw.data, b"tree 0\0");
    assert_eq!(
        String::from(&hash(&empty_raw.data)),
        "4b825dc642cb6eb9a060e54bf8d69288fbee4904"
    );

    // 2. Non-empty tree: verify octal modes (`40000`, `100644`, `100755`) and Git directory sort order
    let oid_a = [0x11u8; 20];
    let oid_b = [0x22u8; 20];
    let oid_c = [0x33u8; 20];
    let tree: Object = Tree::new(vec![
        (EXECUTABLE_FILE_MODE, oid_c.into(), "foo0".to_string()),
        (DIRECTORY_MODE, oid_b.into(), "foo".to_string()),
        (READONLY_FILE_MODE, oid_a.into(), "foo-bar".to_string()),
    ])
    .into();

    let mut expected_body = Vec::new();
    // "foo-bar" ('-') < "foo/" ('/') < "foo0" ('0')
    expected_body.extend_from_slice(b"100644 foo-bar\0");
    expected_body.extend_from_slice(&oid_a);
    expected_body.extend_from_slice(b"40000 foo\0");
    expected_body.extend_from_slice(&oid_b);
    expected_body.extend_from_slice(b"100755 foo0\0");
    expected_body.extend_from_slice(&oid_c);

    let mut expected_raw = format!("tree {}\0", expected_body.len()).into_bytes();
    expected_raw.extend_from_slice(&expected_body);

    let actual_raw = ObjectRaw::from(tree);
    assert_eq!(actual_raw.data, expected_raw);
}

#[test]
fn database_store_uses_2_38_git_layout_and_is_idempotent() {
    let dir = TempDir::new("rit-db-store").unwrap();
    let db = Database::new(dir.path().to_path_buf());

    let oid1 = db
        .store(Blob::new(b"hello\n".to_vec()).into())
        .expect("first store should succeed");
    // let hex = encode_oid_to_hex(&oid1);
    let hex = <String as From<&ObjectId>>::from(&oid1);
    assert_eq!(hex, "ce013625030ba8dba906f756967f9e9ca394464a");

    let expected_path = dir.path().join(&hex[..2]).join(&hex[2..]);
    assert!(
        expected_path.is_file(),
        "Object should be stored at objects/<2hex>/<38hex>, missing {:?}",
        expected_path
    );
    assert!(
        !expected_path.with_added_extension("lock").exists(),
        "No leftover .lock file should remain after Database::store"
    );

    // Decompress stored file and verify exact raw bytes
    let compressed = fs::read(&expected_path).unwrap();
    let mut decoder = ZlibDecoder::new(compressed.as_slice());
    let mut raw = Vec::new();
    decoder.read_to_end(&mut raw).unwrap();
    assert_eq!(raw, b"blob 6\0hello\n");

    // Storing the identical object a second time must succeed idempotently.
    let oid2 = db
        .store(Blob::new(b"hello\n".to_vec()).into())
        .expect("storing an already-existing object must be idempotent");
    assert_eq!(oid1, oid2);
}

// =============================================================================
// Index Parsing, Serialization & Corruption Detection
// =============================================================================

#[test]
fn index_parse_accepts_real_git_index_and_roundtrips() {
    let dir = TempDir::new("rit-index-real-git").unwrap();
    let root = dir.path();

    run_git_checked(root, &["init", "-b", "main"], &[]);
    // Files covering different mod-8 path lengths, subdirectories, and executable mode
    fs::write(root.join("a"), "1").unwrap();
    fs::write(root.join("ab"), "2").unwrap();
    fs::write(root.join("abc"), "3").unwrap();
    fs::create_dir_all(root.join("sub")).unwrap();
    let exe = root.join("sub/run.sh");
    fs::write(&exe, "#!/bin/sh\n").unwrap();
    fs::set_permissions(&exe, fs::Permissions::from_mode(0o755)).unwrap();

    run_git_checked(root, &["add", "-A"], &[]);

    let git_index_bytes = fs::read(root.join(REPOSITORY).join(INDEX)).unwrap();
    let parsed = Index::parse(&git_index_bytes).expect("Index::parse should parse real git index");
    assert_eq!(parsed.entries().len(), 4);

    let paths: Vec<String> = parsed
        .entries()
        .iter()
        .map(|e| e.relative_path().to_string_lossy().into_owned())
        .collect();
    assert_eq!(paths, vec!["a", "ab", "abc", "sub/run.sh"]);
    assert_eq!(*parsed.entries()[0].mode(), 0o100644);
    assert_eq!(*parsed.entries()[3].mode(), 0o100755);

    // Roundtrip: construct a new Index from parsed entries, serialize, and parse back.
    let rebuilt = Index::new(parsed.entries().clone());
    let serialized: Vec<u8> = rebuilt.into();
    let reparsed = Index::parse(&serialized)
        .expect("Index serialized by rit should be parseable by Index::parse");
    assert_eq!(
        common::normalize_index(&reparsed),
        common::normalize_index(&parsed)
    );
}

#[test]
fn index_parse_rejects_corrupted_and_malformed_data() {
    // 1. Too short
    assert!(Index::parse(&[]).is_err());
    assert!(Index::parse(&[0u8; 31]).is_err());

    // Build a valid minimal git index via real git to tamper with.
    let dir = TempDir::new("rit-index-corrupt").unwrap();
    let root = dir.path();
    run_git_checked(root, &["init", "-b", "main"], &[]);
    fs::write(root.join("file.txt"), "hello").unwrap();
    run_git_checked(root, &["add", "-A"], &[]);
    let valid = fs::read(root.join(REPOSITORY).join(INDEX)).unwrap();
    assert!(Index::parse(&valid).is_ok());

    // 2. Flip a bit in the payload without updating checksum -> must fail checksum check
    let mut bad_payload = valid.clone();
    bad_payload[16] ^= 0x01;
    assert!(
        Index::parse(&bad_payload).is_err(),
        "Corrupted payload byte should fail SHA-1 checksum verification"
    );

    // 3. Flip a bit in the trailing 20-byte checksum
    let mut bad_checksum = valid.clone();
    let last = bad_checksum.len() - 1;
    bad_checksum[last] ^= 0x80;
    assert!(
        Index::parse(&bad_checksum).is_err(),
        "Corrupted trailing checksum should be rejected"
    );

    // Helper to build a minimal 0-entry index with custom signature & version and valid checksum.
    let make_header_only = |sig: [u8; 4], version: u32| -> Vec<u8> {
        let mut raw = Vec::new();
        raw.extend_from_slice(&sig);
        raw.extend_from_slice(&version.to_be_bytes());
        raw.extend_from_slice(&0u32.to_be_bytes());
        let sum = hash(&raw);
        raw.extend_from_slice(&*sum);
        raw
    };

    // 4. Invalid signature with valid checksum
    assert!(
        Index::parse(&make_header_only(*b"XXXX", 2)).is_err(),
        "Non-DIRC signature should be rejected"
    );

    // 5. Unsupported index versions with valid checksum
    for bad_ver in [0u32, 1, 5, 99] {
        assert!(
            Index::parse(&make_header_only(*b"DIRC", bad_ver)).is_err(),
            "Unsupported index version {bad_ver} should be rejected"
        );
    }
}

// =============================================================================
// Reference Parsing
// =============================================================================

#[test]
fn reference_parse_handles_symref_and_oid_and_rejects_invalid() {
    // Symbolic ref (Git format: "ref: refs/heads/main\n")
    let sym = Reference::parse(b"ref: refs/heads/main\n".to_vec())
        .expect("Valid Git symref should parse");
    assert!(sym.is_symref());
    assert!(!sym.is_oid());
    assert_eq!(
        sym.relative_path(),
        Some(&RelativePath::build("refs/heads/main").unwrap())
    );

    // Direct OID ref (Git format: 40 hex chars + "\n")
    let hex = "8108bdc25ef58ffcfe56b8b5e406d64818ddf849";
    let expected_oid = hex.try_into().unwrap();
    let direct = Reference::parse(format!("{hex}\n").into_bytes())
        .expect("Valid 40-char hex OID ref should parse");
    assert!(direct.is_oid());
    assert!(!direct.is_symref());
    assert_eq!(direct.oid(), Some(&expected_oid));

    // Malformed references must be rejected
    let invalid: &[&[u8]] = &[
        b"",
        b"not-a-valid-ref\n",
        b"8108bdc25ef58ffcfe56b8b5e406d64818ddf84\n", // 39 hex chars
        b"ref: /etc/passwd\n",                        // absolute path in symref
    ];
    for raw in invalid {
        assert!(
            Reference::parse(raw.to_vec()).is_err(),
            "Reference::parse should reject {:?}",
            String::from_utf8_lossy(raw)
        );
    }
}

// =============================================================================
// LockFile Atomicity, Concurrency & Cleanup
// =============================================================================

#[test]
fn lockfile_atomic_commit_and_cleanup() {
    let dir = TempDir::new("rit-lock-atomic").unwrap();
    let target = dir.path().join("HEAD");
    let lock_path = dir.path().join("HEAD.lock");

    fs::write(&target, "old content\n").unwrap();

    let mut lf = LockFile::new(&target);
    lf.hold().expect("hold should succeed");
    lf.write_all(b"new content\n").unwrap();
    lf.flush().unwrap();

    // Before commit: target is untouched and .lock exists.
    assert!(lock_path.exists());
    assert_eq!(fs::read_to_string(&target).unwrap(), "old content\n");

    lf.commit()
        .expect("commit should atomically replace target");

    // After commit: target has new content and .lock is gone.
    assert!(!lock_path.exists());
    assert_eq!(fs::read_to_string(&target).unwrap(), "new content\n");
}

#[test]
fn lockfile_dotted_filename_appends_lock_extension() {
    let dir = TempDir::new("rit-lock-dotted").unwrap();
    let target = dir.path().join("config.toml");
    let expected_lock = dir.path().join("config.toml.lock");
    let wrong_lock = dir.path().join("config.lock");

    let mut lf = LockFile::new(&target);
    lf.hold().unwrap();
    assert!(expected_lock.exists(), "Expected config.toml.lock to exist");
    assert!(!wrong_lock.exists(), "Must not replace existing extension");
    lf.write_all(b"key = 1\n").unwrap();
    lf.commit().unwrap();

    assert!(!expected_lock.exists());
    assert_eq!(fs::read_to_string(&target).unwrap(), "key = 1\n");
}

#[test]
fn lockfile_prevents_double_hold_and_cleans_up_on_drop() {
    let dir = TempDir::new("rit-lock-double").unwrap();
    let target = dir.path().join("index");
    let lock_path = dir.path().join("index.lock");
    fs::write(&target, b"original").unwrap();

    {
        let mut lf1 = LockFile::new(&target);
        lf1.hold().unwrap();
        lf1.write_all(b"partial uncommitted write").unwrap();

        // Second holder must fail with AlreadyExists.
        let mut lf2 = LockFile::new(&target);
        let err = lf2
            .hold()
            .expect_err("second LockFile::hold must fail while first is active");
        assert_eq!(err.kind(), ErrorKind::AlreadyExists);
        // Dropping lf2 must NOT delete lf1's active lock file!
        drop(lf2);
        assert!(
            lock_path.exists(),
            "Dropping an unheld LockFile must NOT delete another holder's active .lock file!"
        );
    }

    // After lf1 drops without commit: .lock is cleaned up and original target is intact.
    assert!(!lock_path.exists());
    assert_eq!(fs::read(&target).unwrap(), b"original");

    // A new LockFile can now acquire and commit.
    let mut lf3 = LockFile::new(&target);
    lf3.hold()
        .expect("should be able to re-acquire lock after drop");
    lf3.write_all(b"committed").unwrap();
    lf3.commit().unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"committed");
}

#[test]
fn lockfile_commit_without_hold_fails() {
    let dir = TempDir::new("rit-lock-no-hold").unwrap();
    let target = dir.path().join("ref");
    let lf = LockFile::new(&target);
    assert!(
        lf.commit().is_err(),
        "commit without prior hold must return Err"
    );
    assert!(!target.exists());
}

#[test]
fn lockfile_concurrent_threads_mutual_exclusion() {
    let dir = TempDir::new("rit-lock-threads").unwrap();
    let target = Arc::new(dir.path().join("shared_ref"));
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let target = Arc::clone(&target);
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let mut lf = LockFile::new(target.as_ref());
                match lf.hold() {
                    Ok(()) => {
                        lf.write_all(format!("writer-{i}\n").as_bytes()).unwrap();
                        lf.commit().unwrap();
                        true
                    }
                    Err(e) => {
                        assert_eq!(e.kind(), ErrorKind::AlreadyExists);
                        false
                    }
                }
            })
        })
        .collect();

    let winners: usize = handles
        .into_iter()
        .map(|h| usize::from(h.join().unwrap()))
        .sum();
    assert!(winners >= 1, "At least one thread must succeed");

    // Target file must contain a complete, uncorrupted write from one winner.
    let content = fs::read_to_string(target.as_ref()).unwrap();
    assert!(
        content.starts_with("writer-") && content.ends_with('\n'),
        "Concurrent writes must not interleave or corrupt file, got: {:?}",
        content
    );
    assert!(!dir.path().join("shared_ref.lock").exists());
}

// =============================================================================
// RelativePath Validation
// =============================================================================

#[test]
fn relative_path_validation_and_traversal_rejection() {
    assert!(RelativePath::build("a.txt").is_ok());
    assert!(RelativePath::build("dir/sub/file.txt").is_ok());

    // Absolute paths must be rejected.
    assert!(RelativePath::build("/etc/passwd").is_err());
    assert!(RelativePath::build("/").is_err());

    // Parent directory traversal (`..`) must be rejected to prevent path escape attacks.
    assert!(
        RelativePath::build("../escape.txt").is_err(),
        "RelativePath should reject leading '..' traversal"
    );
    assert!(
        RelativePath::build("a/../../escape.txt").is_err(),
        "RelativePath should reject embedded '..' traversal"
    );

    // get_relative_path must reject targets outside base directory.
    let base_dir = TempDir::new("rit-rel-base").unwrap();
    let other_dir = TempDir::new("rit-rel-other").unwrap();
    let outside_file = other_dir.path().join("outside.txt");
    fs::write(&outside_file, "x").unwrap();
    assert!(get_relative_path(base_dir.path(), &outside_file).is_err());
}
