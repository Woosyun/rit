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
        //create index from workspace
        let index_entries = self
            .repository
            .workspace
            .list_files()?
            .iter()
            .map(|path| {
                let content = fs::read(&path)?;

                let blob = Blob::new(content);
                let oid = self.repository.database.store(blob.into())?;
                let metadata = fs::metadata(&path)?;
                let relative_path = self.repository.workspace.relative_path(path)?;

                Ok(IndexEntry::from((metadata, oid, relative_path)))
            })
            .collect::<io::Result<Vec<IndexEntry>>>()?;

        // Change index to reflect structure of work tree
        let mut index_to_tree = IndexToTree::new();
        for index_entry in &index_entries {
            index_to_tree.insert(index_entry)?;
        }

        // store IndexToTree to Tree
        let (_, tree_oid, _) = self.store_tree("/".to_string(), index_to_tree)?;

        // create commit
        //let parent = self.repository.get_head()?;
        let commit = Commit::new(
            tree_oid,
            Default::default(),
            //parent,
            self.author,
            self.committer,
            self.message,
        );
        let commit_oid = self.repository.database.store(commit.into())?;

        // update head
        let head = self.repository.read_head()?;
        self.repository.references.write(head, commit_oid)?;

        //update index
        let index = Index::from(index_entries);
        self.repository.write_index(index)?;

        Ok(())
    }

    // This method stores Tree,
    // creates TreeEntry with ObjectId returned.
    fn store_tree(&self, name: String, index_to_tree: IndexToTree) -> io::Result<TreeEntry> {
        let curr = match index_to_tree {
            IndexToTree::Tree(m) => {
                let entries = m
                    .into_iter()
                    .map(|(name, sub_itt)| self.store_tree(name, sub_itt))
                    .collect::<io::Result<Vec<TreeEntry>>>()?;
                let tree = Tree::new(entries);
                let oid = self.repository.database.store(tree.into())?;

                (DIRECTORY_MODE, oid, name) as TreeEntry
            }
            IndexToTree::Entry(tree_entry) => tree_entry,
        };
        Ok(curr)
    }
}

enum IndexToTree {
    Tree(collections::HashMap<String, IndexToTree>),
    Entry(TreeEntry),
}
impl IndexToTree {
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
        if let IndexToTree::Tree(m) = self {
            if parents.len() == 0 {
                //todo: optimize dealing with name of entry.
                m.insert(tree_entry.2.clone(), IndexToTree::Entry(tree_entry));
            } else {
                let parent = parents.pop_front().unwrap();
                m.entry(parent)
                    .or_insert_with(|| IndexToTree::new())
                    .insert_recursively(parents, tree_entry);
            }
        }
    }
}
