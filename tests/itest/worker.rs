use std::{
    path::{PathBuf, Path},
    collections::HashSet,
    fs,
    io::Write,
};
use rit::prelude::*;
use super::driver::Driver;
use rand::prelude::*;

pub trait Worker: Driver {
    fn added(&mut self) -> &mut HashSet<PathBuf>;
    fn modified(&mut self) -> &mut HashSet<PathBuf>;
    fn removed(&mut self) -> &mut HashSet<PathBuf>;

    fn add(&mut self) -> Result<PathBuf> {
        let index = format!("{}.txt", rand::rng().random::<u32>());
        let index = Path::new(&index);
        self.added().insert(index.to_path_buf());
        let path = self.workdir().join(index);
        let content = format!("{:?}: newly created for integration test", index);
        fs::write(&path, &content)
            .map_err(|e| Error::Workspace(e.to_string()))?;

        Ok(index.to_path_buf())
    }
    fn modify(&mut self, index: &Path) -> Result<()> {
        self.modified().insert(index.to_path_buf());
        let path = self.workdir().join(index);

        let mut fd = fs::OpenOptions::new().append(true).open(&path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        writeln!(fd, "{}", "\n//modified for integration testing")
            .map_err(|e| Error::Workspace(e.to_string()))?;

        Ok(())
    }
    fn remove(&mut self, index: &Path) -> Result<()> {
        self.removed().insert(index.to_path_buf());

        let path = self.workdir().join(index);
        fs::remove_file(&path)
            .map_err(|e| Error::Workspace(e.to_string()))?;
        Ok(())
    }

    fn shuffle_files(&self) -> Result<Vec<PathBuf>> {
        let rev = self.workspace()?.into_rev()?;
        let mut rng = rand::rng();
        let mut indices = rev.0.keys().cloned().collect::<Vec<_>>();
        indices.shuffle(&mut rng);
        Ok(indices)
    }

    fn work_random(&mut self) -> Result<()> {
        let indices = self.shuffle_files()?;
        let number_to_touch = indices.len()/3;
        for (i, index) in indices.iter().enumerate() {
            if i < number_to_touch {
                self.remove(index)?;
            } else if i < 2 * number_to_touch {
                self.modify(index)?;
            } else {
                break;
            }
        }

        let number_of_creation = if indices.len() < 10 {
            10
        } else {
            number_to_touch
        };
        for _ in 0..number_of_creation {
            self.add()?;
        }

        Ok(())
    }
}
