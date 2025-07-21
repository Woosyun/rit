use tempdir::TempDir;
use crate::prelude::*;

#[test]
fn test1() -> Result<()> {
    let tempdir = TempDir::new("load-and-save-ignore-file")
        .map_err(|e| Error::Workspace(e.to_string()))?;

    {
        let mut ignore = Ignore::load(tempdir.path())?;
        ignore.add(".rit".into());
        ignore.add("target".into());
    }

    let ignore = Ignore::load(tempdir.path())?;
    assert!(ignore.is_ignored(&".rit".into()));
    assert!(ignore.is_ignored(&"target".into()));

    Ok(())
}
