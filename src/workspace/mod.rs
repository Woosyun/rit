use std::{error, fs, io, path};

pub struct Workspace {
    working_dir: path::PathBuf,
}
impl Workspace {
    pub fn new<P: AsRef<path::Path>>(working_dir: P) -> io::Result<Self> {
        Ok(Self {
            working_dir: fs::canonicalize(working_dir.as_ref())?,
        })
    }

    pub fn list_files(&self) -> Result<Vec<path::PathBuf>, Box<dyn error::Error>> {
        let mut hash = vec![];
        self.read_dir(&self.working_dir, &mut hash)?;

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
                let relative_path = target_path.strip_prefix(&self.working_dir)?.to_path_buf();
                hash.push(relative_path);
            }
        }

        Ok(())
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
            .unwrap()
            .list_files()
            .unwrap()
            .into_iter()
            .collect::<collections::HashSet<path::PathBuf>>();
        assert_eq!(expected, real);
    }
}
