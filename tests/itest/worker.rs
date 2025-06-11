use std::{
    path::{PathBuf, Path},
    collections::HashSet,
};
use rit::prelude::*;
use super::{
    utils,
    fs,
    driver::Driver,
};
use rand::prelude::*;

pub trait Worker: Driver {
    fn added(&mut self) -> &mut HashSet<PathBuf>;
    fn modified(&mut self) -> &mut HashSet<PathBuf>;
    fn removed(&mut self) -> &mut HashSet<PathBuf>;

    fn add(&mut self, number_of_creation: usize) -> Result<HashSet<PathBuf>> {
        let mut rng = rand::rng();
        let mut files = HashSet::new();

        for i in 0..number_of_creation {
            let new_file = format!("new_file_{}_{}.txt", i, rng.random::<u32>());
            let new_file = Path::new(&new_file);
            println!("create file {:?}", new_file);
            self.added().insert(new_file.to_path_buf());
            files.insert(new_file.to_path_buf());

            let path = self.workdir().join(new_file);
            let content = format!("{:?}: newly created for integration test", new_file);
            if !path.exists() {
                fs::write(&path, &content)?;
            }
        }

        Ok(files)
    }
    fn modify(&mut self, indices: &Vec<&PathBuf>) -> Result<()> {
        for file in indices {
            println!("modify file: {:?}", file);
            self.modified().insert(file.to_path_buf());

            let path = self.workdir().join(&file);
            fs::appendln(&path, "\n//modified for integration testing")?;
        }
        Ok(())
    }
    fn remove(&mut self, indices: &Vec<&PathBuf>) -> Result<()> {
        for file in indices {
            println!("remove file: {:?}", file);
            self.removed().insert(file.to_path_buf());

            let path = self.workdir().join(file);
            if path.exists() {
                fs::remove_file(&path)?;
            }
        }
        Ok(())
    }

    fn shuffle_files(&self) -> Result<Vec<PathBuf>> {
        let rev = self.workspace()?.into_rev()?;
        let mut rng = rand::rng();
        let mut files = rev.0.keys().cloned().collect::<Vec<_>>();
        files.shuffle(&mut rng);
        Ok(files)
    }

    fn work_random(&mut self) -> Result<()> {
        utils::sleep_1_sec();

        let files = self.shuffle_files()?;
        let number_to_touch = files.len()/3;
        let mut files_to_modify = Vec::with_capacity(number_to_touch);
        let mut files_to_remove = Vec::with_capacity(number_to_touch);

        for (i, file) in files.iter().enumerate() {
            if i < number_to_touch {
                files_to_remove.push(file);
            } else if i < 2 * number_to_touch {
                files_to_modify.push(file);
            } else {
                break;
            }
        }

        self.remove(&files_to_remove)?;
        self.modify(&files_to_modify)?;

        //add
        let number_of_creation = if files.len() < 10 {
            10
        } else {
            number_to_touch
        };
        self.add(number_of_creation)?;

        Ok(())
    }
}
