#![allow(dead_code)]

use flate2::read::ZlibDecoder;
use rit::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::Read;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{fs, io};
use tempdir::TempDir;

pub type TestResult<T> = Result<T, Box<dyn Error>>;

/// Fixed author/committer date matching `rit::commit::Builder`'s current timestamp (`0`).
/// If `commit::Builder` gains a configurable timestamp in the future, update both here
/// and in the builder call inside `TestHarness`.
const FIXED_GIT_DATE: &str = "@0 +0000";

/// Default branch used as fallback when initializing the reference `git` repo.
const DEFAULT_FALLBACK_BRANCH: &str = "master";

/// Differential test harness that mirrors filesystem operations and VCS commands
/// across a `rit` repository and a real `git` reference repository, then asserts
/// full state equivalence and `git` interoperability (`git fsck`, `git ls-files`).
pub struct TestHarness {
    pub rit_workdir: TempDir,
    pub git_workdir: TempDir,
}

impl TestHarness {
    pub fn new() -> Self {
        let rit_workdir =
            TempDir::new("test-for-rit").expect("Failed to create temporary directory for rit");
        let git_workdir =
            TempDir::new("test-for-git").expect("Failed to create temporary directory for git");

        Self {
            rit_workdir,
            git_workdir,
        }
    }

    pub fn rit_path(&self) -> &Path {
        self.rit_workdir.path()
    }

    pub fn git_path(&self) -> &Path {
        self.git_workdir.path()
    }

    // -------------------------------------------------------------------------
    // Mirrored workspace filesystem helpers
    // -------------------------------------------------------------------------

    pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(&self, rel_path: P, content: C) {
        let rel_path = rel_path.as_ref();
        let content = content.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            let full = workdir.join(rel_path);
            if let Some(parent) = full.parent() {
                fs::create_dir_all(parent).expect("Failed to create parent directories");
            }
            fs::write(&full, content).expect("Failed to write test file");
        }
    }

    pub fn write_executable<P: AsRef<Path>, C: AsRef<[u8]>>(&self, rel_path: P, content: C) {
        self.write_file(rel_path.as_ref(), content);
        self.set_mode(rel_path, 0o755);
    }

    pub fn set_mode<P: AsRef<Path>>(&self, rel_path: P, mode: u32) {
        let rel_path = rel_path.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            let full = workdir.join(rel_path);
            fs::set_permissions(&full, fs::Permissions::from_mode(mode))
                .expect("Failed to set file permissions");
        }
    }

    pub fn create_dir<P: AsRef<Path>>(&self, rel_path: P) {
        let rel_path = rel_path.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            fs::create_dir_all(workdir.join(rel_path)).expect("Failed to create directory");
        }
    }

    pub fn remove_file<P: AsRef<Path>>(&self, rel_path: P) {
        let rel_path = rel_path.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            fs::remove_file(workdir.join(rel_path)).expect("Failed to remove file");
        }
    }

    pub fn remove_dir_all<P: AsRef<Path>>(&self, rel_path: P) {
        let rel_path = rel_path.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            fs::remove_dir_all(workdir.join(rel_path)).expect("Failed to remove directory");
        }
    }

    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) {
        let from = from.as_ref();
        let to = to.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            let dst = workdir.join(to);
            if let Some(parent) = dst.parent() {
                fs::create_dir_all(parent).expect("Failed to create destination parent dir");
            }
            fs::rename(workdir.join(from), dst).expect("Failed to rename path");
        }
    }

    pub fn create_symlink<P: AsRef<Path>, Q: AsRef<Path>>(&self, target: P, link_path: Q) {
        let target = target.as_ref();
        let link_path = link_path.as_ref();
        for workdir in [self.rit_path(), self.git_path()] {
            let full_link = workdir.join(link_path);
            if let Some(parent) = full_link.parent() {
                fs::create_dir_all(parent).expect("Failed to create symlink parent dir");
            }
            symlink(target, &full_link).expect("Failed to create symlink");
        }
    }

    // -------------------------------------------------------------------------
    // Mirrored reference manipulation helpers
    // -------------------------------------------------------------------------

    /// Sets `.git/HEAD` in both repositories to a symbolic reference (`ref: refs/heads/<branch>\n`).
    pub fn set_head_branch(&self, branch: &str) {
        let content = format!("ref: refs/heads/{branch}\n");
        for workdir in [self.rit_path(), self.git_path()] {
            fs::write(workdir.join(REPOSITORY).join(HEAD), &content)
                .expect("Failed to write HEAD symref");
        }
    }

    /// Writes a custom symbolic ref file (e.g. `refs/heads/alias` -> `ref: refs/heads/real\n`)
    /// in both repositories.
    pub fn write_symref<P: AsRef<Path>>(&self, repo_rel_path: P, target: &str) {
        let repo_rel_path = repo_rel_path.as_ref();
        let content = format!("ref: {target}\n");
        for workdir in [self.rit_path(), self.git_path()] {
            let full = workdir.join(REPOSITORY).join(repo_rel_path);
            if let Some(parent) = full.parent() {
                fs::create_dir_all(parent).expect("Failed to create ref parent dir");
            }
            fs::write(full, &content).expect("Failed to write symbolic ref");
        }
    }

    /// Detaches `.git/HEAD` in both repositories to the current commit OID.
    pub fn detach_head_to_current(&self) -> String {
        let oid = resolve_head_oid(&self.git_path().join(REPOSITORY))
            .expect("Expected HEAD to resolve to a commit OID before detaching");
        let content = format!("{oid}\n");
        for workdir in [self.rit_path(), self.git_path()] {
            fs::write(workdir.join(REPOSITORY).join(HEAD), &content)
                .expect("Failed to detach HEAD");
        }
        oid
    }

    pub fn head_oid_rit(&self) -> TestResult<String> {
        resolve_head_oid(&self.rit_path().join(REPOSITORY))
    }

    pub fn head_oid_git(&self) -> TestResult<String> {
        resolve_head_oid(&self.git_path().join(REPOSITORY))
    }

    // -------------------------------------------------------------------------
    // Command execution (rit + real git oracle)
    // -------------------------------------------------------------------------

    pub fn try_rit_init(&self) -> TestResult<()> {
        init::Builder::new()
            .working_directory(self.rit_path())
            .build()?
            .execute()
    }

    pub fn try_rit_commit(
        &self,
        msg: &str,
        author_name: &str,
        author_email: &str,
        committer_name: &str,
        committer_email: &str,
    ) -> TestResult<()> {
        commit::Builder::new()
            .working_directory(self.rit_path())
            .message(msg)
            .author(author_name, author_email)
            .committer(committer_name, committer_email)
            .build()?
            .execute()
    }

    /// Runs `rit init` and `git init` on their respective workdirs, then verifies
    /// full repository state equivalence and `git fsck`.
    pub fn init(&self) {
        self.try_rit_init().expect("rit init failed");

        // Align the reference git repo's initial branch with whatever valid default
        // branch name `rit init` wrote into `.git/HEAD` (e.g. `main` or `master`).
        let branch = detect_head_branch(&self.rit_path().join(REPOSITORY))
            .unwrap_or_else(|| DEFAULT_FALLBACK_BRANCH.to_string());

        run_git_checked(self.git_path(), &["init", "-b", &branch], &[]);

        self.assert_repo_state_equal();
        self.assert_git_fsck_clean();
    }

    /// Re-initializes both already-initialized repositories and verifies that
    /// existing state (HEAD, refs, objects, index) is preserved identically.
    pub fn reinit(&self) {
        self.try_rit_init().expect("rit re-init failed");
        run_git_checked(self.git_path(), &["init"], &[]);

        self.assert_repo_state_equal();
        self.assert_git_fsck_clean();
    }

    /// Runs `rit commit` and equivalent `git add -A && git commit` on their
    /// respective workdirs, then verifies state equivalence and `git` interop.
    pub fn commit(&self, msg: &str, name: &str, email: &str) {
        self.commit_with_committer(msg, name, email, name, email);
    }

    pub fn commit_with_committer(
        &self,
        msg: &str,
        author_name: &str,
        author_email: &str,
        committer_name: &str,
        committer_email: &str,
    ) {
        self.try_rit_commit(
            msg,
            author_name,
            author_email,
            committer_name,
            committer_email,
        )
        .expect("rit commit failed");

        let env = [
            ("GIT_AUTHOR_NAME", author_name),
            ("GIT_AUTHOR_EMAIL", author_email),
            ("GIT_AUTHOR_DATE", FIXED_GIT_DATE),
            ("GIT_COMMITTER_NAME", committer_name),
            ("GIT_COMMITTER_EMAIL", committer_email),
            ("GIT_COMMITTER_DATE", FIXED_GIT_DATE),
        ];

        run_git_checked(self.git_path(), &["add", "-A"], &env);
        run_git_checked(
            self.git_path(),
            &[
                "commit",
                "--allow-empty",
                "--allow-empty-message",
                "--cleanup=verbatim",
                "-m",
                msg,
            ],
            &env,
        );

        self.assert_repo_state_equal();
        self.assert_git_interop();
    }

    // -------------------------------------------------------------------------
    // State verification & Git interoperability assertions
    // -------------------------------------------------------------------------

    pub fn assert_repo_state_equal(&self) {
        let rit = self.rit_path().join(REPOSITORY);
        let git = self.git_path().join(REPOSITORY);

        // 0. Ensure no stale `.lock` files are left anywhere in rit's `.git` directory.
        let lock_files = find_lock_files(&rit).expect("Failed to scan rit .git for lock files");
        assert!(
            lock_files.is_empty(),
            "Leftover .lock files found in rit repository: {:?}",
            lock_files
        );

        // 1. Compare HEAD (must exist and match byte-for-byte)
        let rit_head = fs::read_to_string(rit.join(HEAD))
            .unwrap_or_else(|e| panic!("Failed to read rit .git/HEAD: {e}"));
        let git_head = fs::read_to_string(git.join(HEAD))
            .unwrap_or_else(|e| panic!("Failed to read git .git/HEAD: {e}"));
        assert_eq!(rit_head, git_head, "HEAD mismatch between rit and git");

        // 2. Compare References under refs/heads/**
        let rit_refs =
            read_references(&rit).unwrap_or_else(|e| panic!("Failed to read rit refs/heads: {e}"));
        let git_refs =
            read_references(&git).unwrap_or_else(|e| panic!("Failed to read git refs/heads: {e}"));
        assert_eq!(
            rit_refs, git_refs,
            "refs/heads mismatch between rit and git"
        );

        // 3. Compare decompressed Objects & verify SHA-1 integrity of every object
        let rit_objects =
            read_objects(&rit).unwrap_or_else(|e| panic!("Failed to read rit objects: {e}"));
        let git_objects =
            read_objects(&git).unwrap_or_else(|e| panic!("Failed to read git objects: {e}"));

        if rit_objects != git_objects {
            let rit_hex: Vec<String> = rit_objects.keys().map(String::from).collect();
            let git_hex: Vec<String> = git_objects.keys().map(String::from).collect();
            let mut details = format!(
                "Objects mismatch between rit and git!\n  rit OIDs ({}): {:?}\n  git OIDs ({}): {:?}\n",
                rit_hex.len(),
                rit_hex,
                git_hex.len(),
                git_hex
            );
            for (oid, raw) in &rit_objects {
                if !git_objects.contains_key(oid) {
                    details.push_str(&format!(
                        "  [only in rit] {} => {:?}\n",
                        String::from(oid),
                        String::from_utf8_lossy(raw)
                    ));
                }
            }
            for (oid, raw) in &git_objects {
                if !rit_objects.contains_key(oid) {
                    details.push_str(&format!(
                        "  [only in git] {} => {:?}\n",
                        String::from(oid),
                        String::from_utf8_lossy(raw)
                    ));
                }
            }
            panic!("{details}");
        }

        // 4. Compare Index (existence + parsed normalized entries)
        let rit_index_exists = rit.join(INDEX).exists();
        let git_index_exists = git.join(INDEX).exists();
        assert_eq!(
            rit_index_exists, git_index_exists,
            "Index existence mismatch (rit={rit_index_exists}, git={git_index_exists})"
        );
        if rit_index_exists {
            let rit_index =
                read_index(&rit).unwrap_or_else(|e| panic!("Failed to parse rit .git/index: {e}"));
            let git_index =
                read_index(&git).unwrap_or_else(|e| panic!("Failed to parse git .git/index: {e}"));
            assert_eq!(
                normalize_index(&rit_index),
                normalize_index(&git_index),
                "Normalized index entries mismatch between rit and git"
            );
        }
    }

    pub fn assert_git_fsck_clean(&self) {
        let out = run_git(self.rit_path(), &["fsck", "--full", "--strict"], &[]);
        assert!(
            out.status.success(),
            "git fsck --full --strict failed on rit repository!\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        );
    }

    pub fn assert_git_interop(&self) {
        self.assert_git_fsck_clean();

        if self.rit_path().join(REPOSITORY).join(INDEX).exists() {
            let rit_ls = run_git_checked(self.rit_path(), &["ls-files", "--stage"], &[]);
            let git_ls = run_git_checked(self.git_path(), &["ls-files", "--stage"], &[]);
            assert_eq!(
                String::from_utf8_lossy(&rit_ls.stdout),
                String::from_utf8_lossy(&git_ls.stdout),
                "`git ls-files --stage` output mismatch on rit vs git repository"
            );
        }

        let rit_status = run_git_checked(self.rit_path(), &["status", "--porcelain"], &[]);
        let git_status = run_git_checked(self.git_path(), &["status", "--porcelain"], &[]);
        assert_eq!(
            String::from_utf8_lossy(&rit_status.stdout),
            String::from_utf8_lossy(&git_status.stdout),
            "`git status --porcelain` output mismatch on rit vs git repository"
        );
    }
}

// -----------------------------------------------------------------------------
// Public helpers for direct git CLI & repo inspection in tests
// -----------------------------------------------------------------------------

pub fn run_git(workdir: &Path, args: &[&str], extra_env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(workdir)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null");
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    cmd.output().expect("Failed to execute git binary")
}

pub fn run_git_checked(workdir: &Path, args: &[&str], extra_env: &[(&str, &str)]) -> Output {
    let out = run_git(workdir, args, extra_env);
    assert!(
        out.status.success(),
        "git {:?} failed in {:?}\nstdout: {}\nstderr: {}",
        args,
        workdir,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    out
}

pub fn git_commit_env<'a>(
    author_name: &'a str,
    author_email: &'a str,
    committer_name: &'a str,
    committer_email: &'a str,
) -> [(&'static str, &'a str); 6] {
    [
        ("GIT_AUTHOR_NAME", author_name),
        ("GIT_AUTHOR_EMAIL", author_email),
        ("GIT_AUTHOR_DATE", FIXED_GIT_DATE),
        ("GIT_COMMITTER_NAME", committer_name),
        ("GIT_COMMITTER_EMAIL", committer_email),
        ("GIT_COMMITTER_DATE", FIXED_GIT_DATE),
    ]
}

pub fn read_references(repo: &Path) -> TestResult<BTreeMap<RelativePath, String>> {
    let mut m = BTreeMap::new();
    let refs_dir = repo.join(REFS).join(HEADS);
    if !refs_dir.is_dir() {
        return Err(format!("refs/heads directory does not exist at {:?}", refs_dir).into());
    }

    let refs_dir_canon = fs::canonicalize(&refs_dir)?;
    visit_dirs(&refs_dir_canon, &mut |dir_entry: &fs::DirEntry| {
        let path = dir_entry.path();
        let relative_path = get_relative_path(&refs_dir_canon, &path)?;
        let data = fs::read_to_string(&path)?;
        m.insert(relative_path, data);
        Ok(())
    })?;

    Ok(m)
}

pub fn read_objects(repo: &Path) -> TestResult<BTreeMap<ObjectId, Vec<u8>>> {
    let mut m = BTreeMap::new();
    let db_dir = repo.join(DATABASE);
    if !db_dir.is_dir() {
        return Err(format!("objects directory does not exist at {:?}", db_dir).into());
    }

    let db_dir_canon = fs::canonicalize(&db_dir)?;
    for entry in fs::read_dir(&db_dir_canon)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Git creates empty `info` and `pack` directories inside `.git/objects`.
        if entry.file_type()?.is_dir() && (name_str == "info" || name_str == "pack") {
            continue;
        }

        if !entry.file_type()?.is_dir() {
            return Err(format!(
                "Unexpected file directly under objects/: {:?}",
                entry.path()
            )
            .into());
        }
        if name_str.len() != 2 || !name_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(format!(
                "Invalid object fan-out directory name '{}' (expected 2 hex chars)",
                name_str
            )
            .into());
        }

        for obj_entry in fs::read_dir(entry.path())? {
            let obj_entry = obj_entry?;
            let obj_name = obj_entry.file_name();
            let obj_name_str = obj_name.to_string_lossy();
            if obj_name_str.len() != 38 || !obj_name_str.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!(
                    "Invalid object filename '{}/{}' (expected 2/38 hex chars)",
                    name_str, obj_name_str
                )
                .into());
            }

            let oid_str = format!("{name_str}{obj_name_str}");
            let oid = ObjectId::try_from(oid_str.as_str())?;

            let compressed = fs::read(obj_entry.path())?;
            let mut decoder = ZlibDecoder::new(compressed.as_slice());
            let mut decompressed = Vec::new();
            decoder
                .read_to_end(&mut decompressed)
                .map_err(|e| format!("Failed to zlib-decompress object {}: {}", oid_str, e))?;

            let actual_oid = hash(&decompressed);
            if actual_oid != oid {
                return Err(format!(
                    "SHA-1 mismatch for object {}: content hashes to {}",
                    oid_str,
                    String::from(&actual_oid)
                )
                .into());
            }

            m.insert(oid, decompressed);
        }
    }

    Ok(m)
}

pub fn read_index(repo: &Path) -> TestResult<Index> {
    let path = repo.join(INDEX);
    let content = fs::read(path)?;
    let index = Index::parse(&content)?;
    Ok(index)
}

#[derive(Debug, Eq, PartialEq)]
pub struct NormalizedIndexEntry {
    pub path: RelativePath,
    pub mode: u32,
    pub oid: ObjectId,
}

pub fn normalize_index(index: &Index) -> Vec<NormalizedIndexEntry> {
    index
        .entries()
        .iter()
        .map(|e| NormalizedIndexEntry {
            path: e.relative_path().clone(),
            mode: *e.mode(),
            oid: e.oid().clone(),
        })
        .collect()
}

pub fn resolve_head_oid(repo: &Path) -> TestResult<String> {
    let mut current_path = repo.join(HEAD);
    for _ in 0..16 {
        let raw = fs::read_to_string(&current_path)?;
        let trimmed = raw.trim_end_matches('\n');
        if let Some(target) = trimmed.strip_prefix("ref: ") {
            current_path = repo.join(target);
        } else {
            if trimmed.len() != 40 || !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("Invalid OID in {:?}: {:?}", current_path, raw).into());
            }
            return Ok(trimmed.to_string());
        }
    }
    Err("Symbolic ref chain exceeded maximum depth".into())
}

fn detect_head_branch(repo: &Path) -> Option<String> {
    let head = fs::read_to_string(repo.join(HEAD)).ok()?;
    let stripped = head.strip_prefix("ref: refs/heads/")?.strip_suffix('\n')?;
    if stripped.is_empty() {
        None
    } else {
        Some(stripped.to_string())
    }
}

fn find_lock_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut locks = Vec::new();
    fn walk(dir: &Path, locks: &mut Vec<PathBuf>) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                walk(&path, locks)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("lock") {
                locks.push(path);
            }
        }
        Ok(())
    }
    walk(dir, &mut locks)?;
    Ok(locks)
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&fs::DirEntry) -> TestResult<()>) -> TestResult<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}
