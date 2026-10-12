use crate::{database, repository, workspace};
use std::{collections, error, fs, io, path};

pub struct Commit {
    ws: workspace::Workspace,
    db: database::Database,
}
impl Commit {
    pub fn new<P: AsRef<path::Path>>(workdir: P) -> Result<Self, Box<dyn error::Error>> {
        let workdir = workdir.as_ref().canonicalize()?;
        let db = database::Database::new(workdir.join(repository::DEFAULT_NAME));
        let ws = workspace::Workspace::new(workdir)?;

        Ok(Commit { ws, db })
    }
    fn set_parent(&self) -> io::Result<database::Oid> {
        todo!()
    }
    fn set_message(self, message: String) -> () {
        todo!()
    }
    fn set_author(self, name: String, email: String, timestamp: u64) {
        todo!()
    }
    fn set_committer(self, name: String, email: String, timestamp: u64) {
        todo!();
    }

    pub fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let mut entries = vec![];
        for path in self.ws.list_files()? {
            let content = fs::read(&path)?;
            let blob = database::Blob::new(content);
            let oid = self.db.store(blob)?;
            let metadata = fs::metadata(&path)?;
            let relative_path = self.ws.relative_path(path)?;

            //let entry = index::Entry::new(relative_path, oid, metadata);
            //entries.push(entry);
            entries.push((metadata, relative_path, oid));
        }
        //todo: update index
        //tothink: it is important to decide where to sort

        let mut rectree = RecTree::new();
        for (metadata, relative_path, oid) in entries.into_iter() {
            rectree.insert(metadata, relative_path, oid);
        }
        let (_, root_oid, _) = RecEntry::Tree(rectree).store("/".to_string(), &self.db)?;

        //let commit = database::Commit::new(root_oid,

        Ok(())
    }
}

type FileName = String;
struct RecTree {
    entries: collections::HashMap<FileName, RecEntry>,
}
impl RecTree {
    fn new() -> Self {
        Self {
            entries: collections::HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        metadata: fs::Metadata,
        relative_path: path::PathBuf,
        oid: database::Oid,
    ) {
        let mode = if metadata.permissions().readonly() {
            database::READONLY_FILE_MODE
        } else {
            database::EXECUTABLE_FILE_MODE
        };
        let mut paths = relative_path.components();
        let Some(path::Component::Normal(entry_name)) = paths.next() else {
            return;
        };
        let Some(entry_name) = entry_name.to_str() else {
            return; //not a valid unicode. tothink: is this rigth behaviour?
        };
        self.add(paths, entry_name, mode, oid);
    }
    fn add(
        &mut self,
        mut paths: path::Components,
        entry_name: &str,
        mode: u16,
        oid: database::Oid,
    ) {
        if let Some(path::Component::Normal(next_entry_name)) = paths.next() {
            let Some(next_entry_name) = next_entry_name.to_str() else {
                return;
            };
            //this is tree
            self.entries
                .entry(entry_name.to_string())
                .or_insert_with(|| RecEntry::Tree(RecTree::new()));
            if let Some(RecEntry::Tree(rectree)) = self.entries.get_mut(entry_name) {
                rectree.add(paths, next_entry_name, mode, oid);
            }
        } else {
            self.entries
                .insert(entry_name.to_string(), RecEntry::Entry(mode, oid));
        }
    }

    //database::Tree로 바꿔서 저장하고 database::Oid를 리턴
    fn store(self, db: &database::Database) -> Result<database::Oid, Box<dyn error::Error>> {
        let db_entries = self
            .entries
            .into_iter()
            .map(|(filename, recentry)| recentry.store(filename, db))
            .collect::<Result<Vec<_>, Box<dyn error::Error>>>()?;
        let db_tree = database::Tree::from(db_entries);
        db.store(db_tree)
    }
}
enum RecEntry {
    Tree(RecTree),
    Entry(u16, database::Oid),
}
impl RecEntry {
    pub fn store(
        self,
        filename: String,
        db: &database::Database,
    ) -> Result<database::Entry, Box<dyn error::Error>> {
        let entry = match self {
            RecEntry::Tree(rectree) => {
                let oid = rectree.store(db)?;
                (database::DIRECTORY_MODE, oid, filename)
            }
            RecEntry::Entry(mode, oid) => (mode, oid, filename) as database::Entry,
        };
        Ok(entry)
    }
}
