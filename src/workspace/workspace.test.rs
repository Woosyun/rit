use crate::prelude::*;
use tempdir::TempDir;
use std::fs;

#[test]
fn get_ancestors() -> Result<()> {
    let tempdir = TempDir::new("get-ancestors-of-file")
        .map_err(|e| Error::Workspace(e.to_string()))?;

    let mut path = tempdir.path().to_path_buf();
    path.push("dir1");
    fs::create_dir(&path).unwrap();
    path.push("dir2");
    fs::create_dir(&path).unwrap();
    path.push("file1.txt");

    fs::write(&path, "file1")
        .map_err(|e| Error::Workspace(e.to_string()))?;

    let ws = Workspace::build(tempdir.path().to_path_buf())?;
    let index = ws.get_relative_path(&path)?;
    let components = index
        .components()
        .filter_map(|c| {
            match c {
                std::path::Component::Normal(name) => Some(name),
                _ => None,
            }
        })
        .map(|oss| oss.to_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(components.get(0).unwrap(), "dir1");
    assert_eq!(components.get(1).unwrap(), "dir2");

    Ok(())
}

#[test]
fn list_files() -> Result<()> {
    let tempdir = TempDir::new("read-workspace-and-list-files")
        .map_err(|e| Error::Workspace(e.to_string()))?;
    
    let create_file = || {

    };

    Ok(())
}
