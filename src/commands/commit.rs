use crate::prelude::*;
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
    repository: Repository,
    message: String,
    author: Author,
    committer: Committer,
}
impl Command<Commit, CommitBuilder> for Commit {
    fn builder() -> CommitBuilder {
        CommitBuilder::new()
    }
    fn execute(self) -> Result<(), Box<dyn error::Error>> {
        //create index from workspace
        let index = self
            .repository
            .workspace
            .list_files()?
            .iter()
            .map(|path| {
                let content = fs::read(&path)?;

                let blob = Blob::new(content);
                let oid = self.repository.database.store(blob)?;
                let metadata = fs::metadata(&path)?;
                let relative_path = self.repository.workspace.relative_path(path)?;

                Ok((relative_path, metadata, oid))
            })
            .collect::<Result<Index, Box<dyn error::Error>>>()?;

        //create tree from index
        let mut rectree = RecTree::new();
        for index_entry in index.entries() {
            rectree.insert(index_entry.mode(), index_entry.name(), index_entry.oid());
        }
        let (_, root_oid, _) = RecEntry::Tree(rectree).store("/".to_string(), &self.repository)?;

        /*
        let parent = self.repository.refs.read_head()?.oid()?.clone();
        let commit =
            object::Commit::new(root_oid, parent, self.author, self.committer, self.message);
        let new_head = self.db.store(commit)?;
        */
        //self.refs.update_head(new_head)?;

        todo!("update index");
        Ok(())
    }
}

#[derive(Default)]
pub struct CommitBuilder {
    workdir: Option<path::PathBuf>,
    message: Option<String>,
    author: Option<Author>,
    committer: Option<Committer>,
}
impl CommitBuilder {
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn author(mut self, name: &str, email: &str) -> Self {
        let now = 0;
        self.author = Some(Author::new(name.into(), email.into(), now));
        self
    }
    pub fn committer(mut self, name: &str, email: &str) -> Self {
        let now = 0;
        self.committer = Some(Committer::new(name.into(), email.into(), now));
        self
    }
}
impl CommandBuilder<Commit> for CommitBuilder {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    fn working_directory<P: AsRef<path::Path>>(mut self, working_directory: P) -> Self {
        let workdir = working_directory.as_ref().to_path_buf();
        self.workdir = Some(workdir);
        self
    }
    fn build(self) -> io::Result<Commit> {
        let Some(workdir) = self.workdir else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cannot find working directory",
            ));
        };
        let repository = Repository::build(workdir)?;
        let commit = Commit {
            repository,
            message: self
                .message
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "commit message missing"))?,
            author: self.author.ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "author information missing")
            })?,
            committer: self.committer.ok_or_else(|| {
                io::Error::new(io::ErrorKind::NotFound, "committer information missing")
            })?,
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
    fn insert(&mut self, mode: &u32, relative_path: &path::Path, oid: &ObjectId) {
        let mut paths = relative_path.components();
        let Some(path::Component::Normal(entry_name)) = paths.next() else {
            return;
        };
        let Some(entry_name) = entry_name.to_str() else {
            return; //not a valid unicode. tothink: is this rigth behaviour?
        };
        self.add(paths, entry_name, mode.clone(), oid.clone());
    }
    fn add(&mut self, mut paths: path::Components, entry_name: &str, mode: u32, oid: ObjectId) {
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
    fn store(self, repo: &Repository) -> Result<ObjectId, Box<dyn error::Error>> {
        let db_entries = self
            .entries
            .into_iter()
            .map(|(filename, recentry)| recentry.store(filename, repo))
            .collect::<Result<Vec<TreeEntry>, Box<dyn error::Error>>>()?;
        let db_tree = Tree::new(db_entries);
        repo.database.store(db_tree).map_err(Into::into)
    }
}

enum RecEntry {
    Tree(RecTree),
    Entry(u32, ObjectId),
}
impl RecEntry {
    fn store(self, filename: String, db: &Repository) -> Result<TreeEntry, Box<dyn error::Error>> {
        let entry = match self {
            RecEntry::Tree(rectree) => {
                let oid = rectree.store(db)?;
                (DIRECTORY_MODE, oid, filename)
            }
            RecEntry::Entry(mode, oid) => (mode, oid, filename).into(),
        };
        Ok(entry)
    }
}
