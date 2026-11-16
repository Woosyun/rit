use crate::prelude::*;
use std::{collections, error, fs, io, path};
use tempdir::TempDir;

pub type TestResult<T> = Result<T, Box<dyn error::Error>>;

pub struct TestHarness {
    pub rit_workdir: TempDir,
    pub git_workdir: TempDir,
}
impl TestHarness {
    pub fn build() -> io::Result<Self> {
        let rit_workdir = TempDir::new("test-for-rit")?;
        let git_workdir = TempDir::new("test-for-git")?;

        let th = Self {
            rit_workdir,
            git_workdir,
        };
        Ok(th)
    }

    pub fn try_init(&self) -> TestResult<()> {
        Ok(())
    }

    pub fn assert_repo_state_equal(&self) -> TestResult<bool> {
        let rit = self.rit_workdir.path().join(REPOSITORY);
        let git = self.git_workdir.path().join(REPOSITORY);

        // 1. Compare HEAD
        assert_eq!(
            fs::read_to_string(rit.join(HEAD)).ok(),
            fs::read_to_string(git.join(HEAD)).ok(),
            "HEAD mismatch",
        );

        // 2. Compare References
        assert_eq!(
            read_references(&rit).ok(),
            read_references(&git).ok(),
            "REFS mismatch"
        );
        // 3. Compare Objects
        assert_eq!(
            read_objects(&rit).ok(),
            read_objects(&git).ok(),
            "objects mismatch",
        );

        // 4. Compare Index
        assert_eq!(
            read_index(&rit).ok(),
            read_index(&git).ok(),
            "index mismatch",
        );

        Ok(true)
    }
}

fn read_references(repo: &path::Path) -> TestResult<collections::BTreeMap<RelativePath, String>> {
    let mut m = collections::BTreeMap::new();
    let refs_dir = repo.join(REFS).join(HEADS);

    visit_dirs(&refs_dir, &mut |dir_entry: &fs::DirEntry| {
        if !dir_entry.path().try_exists()? {
            return Ok(());
        }
        let relative_path = get_relative_path(&refs_dir, &dir_entry.path())?;
        let data = fs::read_to_string(dir_entry.path())?;
        m.insert(relative_path, data);
        Ok(())
    })?;

    Ok(m)
}
fn read_objects(repo: &path::Path) -> TestResult<collections::BTreeMap<ObjectId, String>> {
    let mut m = collections::BTreeMap::new();
    let db_dir = repo.join(DATABASE);

    visit_dirs(&db_dir, &mut |dir_entry: &fs::DirEntry| {
        if !dir_entry.path().try_exists()? {
            return Ok(());
        }
        // get two parent components
        let relative_path = get_relative_path(&db_dir, &dir_entry.path())?;
        let oid_str = relative_path
            .components()
            .map(|comp| comp.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("");
        if oid_str.len() != 40 {
            return Err("oid is wrong".into());
        }
        let oid = decode_to_oid(&oid_str)?;
        let data = fs::read_to_string(db_dir.join(dir_entry.path()))?;
        m.insert(oid, data);
        Ok(())
    })?;

    Ok(m)
}
fn read_index(repo: &path::Path) -> TestResult<Index> {
    let path = repo.join(INDEX);
    let content = fs::read(path)?;
    let index = Index::parse(&content)?;
    Ok(index)
}

fn visit_dirs(
    dir: &path::Path,
    cb: &mut dyn FnMut(&fs::DirEntry) -> TestResult<()>,
) -> TestResult<()> {
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
