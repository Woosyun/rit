use std::{
    path::PathBuf,
    io::{self, prelude::*},
    fs,
};
use crate::prelude::*;

pub trait Database {
    fn get_database(&self) -> io::Result<PathBuf>;

    fn store_object<O: Objectify>(&self, obj: O) -> io::Result<()> {
        let hash256 = obj.calculate_hash();
        let object = obj.objectify();

        let (dir, file) = hash256.split();
        let mut path = self.get_database()?;
        path.push(dir);
        let _ = fs::create_dir(&path)?;

        let mut tmp = path.clone();

        path.push(file);
        if path.exists() {
            return Ok(());
        }

        tmp.push(self.tmp_file_name());
        let mut tmp_file = fs::File::create(&tmp)?;
        tmp_file.write_all(object.as_bytes())?;
        let _ = fs::rename(tmp, path)?;

        Ok(())
    }

    fn tmp_file_name<'a>(&'a self) -> &'a str {
        "tmp"
    }
}
