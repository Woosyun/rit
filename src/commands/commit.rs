use crate::commands::command::*;
use crate::{database, index, refs, repository, workspace};
use std::{collections, error, fs, io, path};

/// Commit command.
///
/// ## Usage
/// ```
/// Commit::builder()
///     .message("first commit")
///     .working_directory(env::current_dir()?)
///     .author("me", "me123@gmail.com")
///     .committer("someone", "someone123@gmail.com")
///     .build()
///     .execute()?;
/// ```
pub struct Commit {
    ws: workspace::Workspace,
    db: database::Database,
    refs: refs::Refs,
    message: String,
    author: database::Author,
    committer: database::Committer,
}
impl Command<Commit, CommitBuilder> for Commit {
    fn builder() -> CommitBuilder {
        CommitBuilder::new()
    }
    fn execute(self) -> Result<(), Box<dyn error::Error>> {
        let mut index = index::Index::new(&self.ws.path).load_mut()?;

        for path in self.ws.list_files()? {
            let content = fs::read(&path)?;
            let blob = database::Blob::new(content);
            let oid = self.db.store(blob)?;
            let metadata = fs::metadata(&path)?;
            let relative_path = self.ws.relative_path(path)?;

            //entries.push((metadata, relative_path, oid));
            index.insert(relative_path, metadata, oid);
        }

        let mut rectree = RecTree::new();
        for (relative_path, index_entry) in index.entries().iter() {
            rectree.insert(index_entry.mode(), relative_path, index_entry.oid());
        }
        let (_, root_oid, _) = RecEntry::Tree(rectree).store("/".to_string(), &self.db)?;

        let parent = self.refs.read_head()?.oid()?.clone();
        let commit =
            database::Commit::new(root_oid, parent, self.author, self.committer, self.message);
        let new_head = self.db.store(commit)?;
        //self.refs.update_head(new_head)?;

        Ok(())
    }
}

#[derive(Default)]
pub struct CommitBuilder {
    ws: Option<workspace::Workspace>,
    db: Option<database::Database>,
    refs: Option<refs::Refs>,
    message: Option<String>,
    author: Option<database::Author>,
    committer: Option<database::Committer>,
}
impl CommitBuilder {
    pub fn working_directory<P: AsRef<path::Path>>(
        mut self,
        working_directory: P,
    ) -> io::Result<Self> {
        let workdir = working_directory.as_ref();
        self.ws = Some(workspace::Workspace::new(workdir.to_path_buf())?);
        let repodir = workdir.join(repository::DIR_NAME);
        self.db = Some(database::Database::new(repodir.clone()));
        self.refs = Some(refs::Refs::new(repodir));
        Ok(self)
    }
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn author(mut self, name: &str, email: &str) -> Self {
        let now = 0;
        self.author = Some(database::Author::new(name.into(), email.into(), now));
        self
    }
    pub fn committer(mut self, name: &str, email: &str) -> Self {
        let now = 0;
        self.committer = Some(database::Committer::new(name.into(), email.into(), now));
        self
    }
}
impl CommandBuilder<Commit> for CommitBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn build(self) -> Result<Commit, Box<dyn error::Error>> {
        let commit = Commit {
            ws: self.ws.ok_or_else(|| "no workspace found".to_string())?,
            db: self.db.ok_or_else(|| "no database found".to_string())?,
            refs: self.refs.ok_or_else(|| "no refs found".to_string())?,
            message: self.message.ok_or_else(|| "no message found".to_string())?,
            author: self.author.ok_or_else(|| "no author found".to_string())?,
            committer: self
                .committer
                .ok_or_else(|| "no committer found".to_string())?,
        };
        Ok(commit)
    }
}

// This is used for ease transitions of files and folders
// from workspace to index, to database.
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
    fn insert(&mut self, mode: &u32, relative_path: &path::Path, oid: &database::Oid) {
        let mut paths = relative_path.components();
        let Some(path::Component::Normal(entry_name)) = paths.next() else {
            return;
        };
        let Some(entry_name) = entry_name.to_str() else {
            return; //not a valid unicode. tothink: is this rigth behaviour?
        };
        self.add(paths, entry_name, mode.clone(), oid.clone());
    }
    fn add(
        &mut self,
        mut paths: path::Components,
        entry_name: &str,
        mode: u32,
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
    //rectree and recentry should not have responsibility
    //of storing object in database
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
    Entry(u32, database::Oid),
}
impl RecEntry {
    fn store(
        self,
        filename: String,
        db: &database::Database,
    ) -> Result<database::Entry, Box<dyn error::Error>> {
        let entry = match self {
            RecEntry::Tree(rectree) => {
                let oid = rectree.store(db)?;
                (database::DIRECTORY_MODE, oid, filename)
            }
            RecEntry::Entry(mode, oid) => (mode, oid, filename).into(),
        };
        Ok(entry)
    }
}
