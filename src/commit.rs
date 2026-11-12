use crate::prelude::*;
use std::{collections, error, fs, io, path};

/// ## Usage
/// ```Rust
/// commit::Builder::new()
///     .working_directory(env::current_dir()?)
///     .message("first commit")
///     .author("me", "abc123@gmail.com")
///     .committer("me", "abc123@gmail.com")
///     .build()
/// ```
#[derive(Default)]
pub struct Builder {
    workdir: Option<path::PathBuf>,
    message: Option<String>,
    author: Option<Author>,
    committer: Option<Committer>,
}
impl Builder {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn working_directory<P: AsRef<path::Path>>(mut self, working_directory: P) -> Self {
        self.workdir = Some(working_directory.as_ref().to_path_buf());
        self
    }
    pub fn message(mut self, msg: &str) -> Self {
        self.message = Some(msg.to_string());
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
    pub fn build(self) -> io::Result<Command> {
        let Some(workdir) = self.workdir else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cannot find working directory",
            ));
        };
        let repository = Repository::build(workdir)?;
        //let index = Index::read_from_repository(&repository)?;
        let commit = Command {
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

/// ## Usage
/// ```Rust
/// commit::Builder::new()
///     ...
///     .build()
///     .execute()?;
/// ```
pub struct Command {
    repository: Repository,
    message: String,
    author: Author,
    committer: Committer,
}
impl Command {
    pub fn execute(self) -> Result<(), Box<dyn error::Error>> {
        // 1. Store files and create index from workspace
        let index_entries: Vec<IndexEntry> = self
            .repository
            .workspace
            .list_files()?
            .iter()
            .map(|path| {
                let content = fs::read(&path)?;

                let blob = Blob::new(content);
                let oid = self
                    .repository
                    .database
                    .store(<Object as From<Blob>>::from(blob))?;
                let metadata = fs::metadata(&path)?;
                let relative_path = self.repository.workspace.relative_path(path)?;

                Ok(IndexEntry::new(metadata, oid, relative_path))
            })
            .collect::<io::Result<Vec<IndexEntry>>>()?;
        let index = Index::new(index_entries);

        // 2. Convert Index to Tree and store it recursively
        let mut store_index = StoreIndex::new();
        for index_entry in index.entries() {
            store_index.insert(index_entry)?;
        }
        let (_, tree_oid, _) = self.store_tree("/".to_string(), store_index)?;

        // 3. Create commit
        let mut parent_ref = self.repository.read_head()?;
        while !parent_ref.is_oid() {
            let next_ref = parent_ref
                .relative_path()
                .expect("At least reference should be SymRef or ObjectId");
            parent_ref = self.repository.references.read(next_ref)?;
        }
        let parent_oid = parent_ref.oid().expect("The reference should be ObjectId");
        let commit = Commit::new(
            tree_oid,
            parent_oid.clone(),
            self.author,
            self.committer,
            self.message,
        );
        let commit_oid = self.repository.database.store(commit.into())?;

        // 4. Update head
        let head = self.repository.read_head()?;
        let head = if head.is_oid() {
            RelativePath::build(HEAD)?
        } else {
            head.relative_path()
                .expect("This reference(head) should be SymRef")
                .clone()
        };

        self.repository.references.write(&head, commit_oid)?;
        self.repository.write_index(index)?;

        Ok(())
    }

    // This method stores Tree,
    // creates `TreeEntry` with `ObjectId` returned.
    fn store_tree(&self, name: String, store_index: StoreIndex) -> io::Result<TreeEntry> {
        let curr = match store_index {
            StoreIndex::Tree(m) => {
                let entries = m
                    .into_iter()
                    .map(|(name, sub_si)| self.store_tree(name, sub_si))
                    .collect::<io::Result<Vec<TreeEntry>>>()?;
                let tree = Tree::new(entries);
                let oid = self.repository.database.store(tree.into())?;

                (DIRECTORY_MODE, oid, name) as TreeEntry
            }
            StoreIndex::Entry(tree_entry) => tree_entry,
        };
        Ok(curr)
    }
}

enum StoreIndex {
    Tree(collections::HashMap<String, StoreIndex>),
    Entry(TreeEntry),
}
impl StoreIndex {
    fn new() -> Self {
        Self::Tree(Default::default())
    }

    // This method receives IndexEntry,
    // creates TreeEntry from it,
    // call 'insert_recursively' method to store it.
    fn insert(&mut self, index_entry: &IndexEntry) -> io::Result<()> {
        let mode = index_entry.mode().clone();
        let oid = index_entry.oid().clone();
        let relative_path = index_entry.relative_path();

        let file_name = relative_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "cannot get file name"))?
            .to_string_lossy()
            .to_string();
        let comps = relative_path.components().collect::<Vec<_>>();
        let len_of_parents = comps.len() - 1;
        let parents = comps
            .into_iter()
            .take(len_of_parents)
            .map(|comp| comp.as_os_str().to_string_lossy().to_string()) // tothink: sorting based on path?
            .collect::<collections::VecDeque<String>>();
        let tree_entry: TreeEntry = (mode, oid, file_name);

        self.insert_recursively(parents, tree_entry);

        Ok(())
    }

    // This method recieves TreeEntry,
    // stores it going through IndexToTree recursively.
    fn insert_recursively(
        &mut self,
        mut parents: collections::VecDeque<String>,
        tree_entry: TreeEntry,
    ) {
        if let StoreIndex::Tree(m) = self {
            if parents.len() == 0 {
                //todo: optimize dealing with name of entry.
                m.insert(tree_entry.2.clone(), StoreIndex::Entry(tree_entry));
            } else {
                let parent = parents.pop_front().unwrap();
                m.entry(parent)
                    .or_insert_with(|| StoreIndex::new())
                    .insert_recursively(parents, tree_entry);
            }
        }
    }
}
