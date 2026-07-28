use std::path::{PathBuf, Path};

pub struct Workspace {
    working_directory: PathBuf
}
impl Workspace {
    pub fn new<P: AsRef<Path>>(working_directory: P) -> Self {
        Self {
            working_directory: working_directory.as_ref().to_path_buf(),
        }
    }

    // todo: store files and make them entries to return
    //pub fn store_objects_and_list_entries(&self, db: &Database) -> std::io::Result<Vec<Entry>> {}
}
