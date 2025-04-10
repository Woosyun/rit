use rit::commands::*;
use tempdir::TempDir;

#[test]
pub fn initialize_repository() {
    let tempdir = TempDir::new("initialize_repository").unwrap();
    let workdir = tempdir.path().to_path_buf();

    let init = Init::build(workdir.clone()).unwrap();
    init.execute().unwrap();

    let mut repo = workdir;
    repo.push(".rit");
    assert!(repo.exists());

    let mut db = repo.clone();
    db.push("objects");
    assert!(db.exists());

    let mut refs = repo.clone();
    refs.push("refs");
    assert!(refs.exists());
    refs.push("local");
    assert!(refs.exists());
}
