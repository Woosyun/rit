use crate::prelude::*;
use std::{error, fs, path};

/// Responsible for handle workspace.
/// To read workspace and get file paths, use [`Workspace::list_files`].
pub struct Workspace {
    pub path: path::PathBuf,
}
impl Workspace {
    pub fn new<P: AsRef<path::Path>>(workdir: P) -> Self {
        Self {
            path: workdir.as_ref().to_path_buf(),
        }
    }

    /// This method returns vector of paths representing all files in the workspace.
    ///
    /// **Note** Result is not garanteed to be sorted.
    pub fn list_files(&self) -> Result<Vec<path::PathBuf>, Box<dyn error::Error>> {
        let mut hash = vec![];
        self.read_dir(&self.path, &mut hash)?;

        Ok(hash)
    }
    fn read_dir<P: AsRef<path::Path>>(
        &self,
        dir: P,
        hash: &mut Vec<path::PathBuf>,
    ) -> Result<(), Box<dyn error::Error>> {
        for direntry in fs::read_dir(&dir)? {
            let direntry = direntry?;
            let target_path = direntry.path();

            //todo: filter with .gitignore

            let file_type = direntry.file_type()?;
            if file_type.is_symlink() {
                //tothink: consider how to deal with symlink
                continue;
            } else if file_type.is_dir() {
                self.read_dir(target_path, hash)?;
            } else {
                let relative_path = target_path.strip_prefix(&self.path)?.to_path_buf();
                hash.push(relative_path);
            }
        }

        Ok(())
    }

    /// this method returns relative path from working directory.
    pub fn relative_path<P: AsRef<path::Path>>(
        &self,
        path: P,
    ) -> Result<RelativePath, Box<dyn error::Error>> {
        let re = path.as_ref().strip_prefix(&self.path)?.to_path_buf();
        if re.is_relative() {
            Ok(RelativePath::new(re))
        } else {
            Err("path is not relative path".into())
        }
    }
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn read_correctly() {
        use super::Workspace;
        use std::os::unix::fs::symlink;
        use std::{collections, fs, path};
        use tempdir::TempDir;

        let tempdir = TempDir::new("test_read_correctly").unwrap();

        let workdir = tempdir.path();
        let mut expected = collections::HashSet::new();

        let path1 = workdir.join("a.txt");
        fs::write(&path1, "a").expect("should be able to write to a.txt");

        expected.insert(path::Path::new("a.txt").to_path_buf());

        let dir2 = workdir.join("b");
        fs::create_dir(&dir2).expect("should be able to create directory b/");
        let path2 = dir2.join("c.txt");
        fs::write(&path2, "c").expect("should be able to write to c.txt");

        expected.insert(path::Path::new("b/c.txt").to_path_buf());

        //add symlink which is not valid entry
        symlink(workdir.join("a.txt"), workdir.join("a_link")).unwrap();

        let real = Workspace::new(workdir)
            .list_files()
            .unwrap()
            .into_iter()
            .collect::<collections::HashSet<path::PathBuf>>();
        assert_eq!(expected, real);
    }
}
