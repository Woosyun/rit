use crate::{repository, workspace};
use std::{
    path::{PathBuf, Path},
    collections::{HashMap, HashSet},
    fmt::Write,
};

#[derive(PartialEq, Clone, Debug)]
pub struct Status {
    ws: workspace::Workspace,
    db: repository::Database,
    head: repository::Head,
    refs: repository::Refs,
    ignore: repository::Ignore,
    pub added: HashSet<PathBuf>,
    pub deleted: HashSet<PathBuf>,
    pub modified: HashSet<PathBuf>,
}
impl Status {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = workspace::Workspace::build(workdir)?;
        let repo = repository::Repository::build(&ws)?;
        let db = repo.get_database()?;
        let head = repo.get_head();
        let refs = repo.get_refs();
        let ignore = repo.get_ignore()?;

        let status = Self {
            ws,
            db,
            head,
            refs,
            ignore,
            added: HashSet::new(),
            deleted: HashSet::new(),
            modified: HashSet::new(),
        };

        Ok(status)
    }

    pub fn output(&self) -> String {
        let mut output = String::new();

        writeln!(output, "added files:").unwrap();
        let mut added_files = self.added
            .iter()
            .map(|file| {
                file
                    .to_str().unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>();
        added_files.sort();
        let added_files = added_files.join("\n");
        writeln!(output, "{}", added_files).unwrap();

        writeln!(output, "modified files:").unwrap();
        let mut modified_files = self.modified
            .iter()
            .map(|file| {
                file
                    .to_str().unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>();
        modified_files.sort();
        let modified_files = modified_files.join("\n");
        writeln!(output, "{}", modified_files).unwrap();

        writeln!(output, "deleted files:").unwrap();
        let mut deleted_files = self.deleted
            .iter()
            .map(|file| {
                file
                    .to_str().unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>();
        deleted_files.sort();
        let deleted_files = deleted_files.join("\n");
        writeln!(output, "{}", deleted_files).unwrap();

        output
    }

    fn is_ignored(&self, path: &Path) -> crate::Result<bool> {
        let name = self.ws.get_file_name(&path)?;
        Ok(self.ignore.is_ignored(&name))
    }
    pub fn added(&mut self, path: &Path) -> crate::Result<()>{
        if !self.is_ignored(path)? {
            self.added.insert(self.ws.get_relative_path(path)?);
        }
        Ok(())
    }
    pub fn modified(&mut self, path: &Path) -> crate::Result<()>{
        if !self.is_ignored(path)? {
            self.modified.insert(self.ws.get_relative_path(path)?);
        }
        Ok(())
    }
    pub fn deleted(&mut self, path: &Path) -> crate::Result<()>{
        self.deleted.insert(self.ws.get_relative_path(path)?);
        Ok(())
    }

    pub fn execute(&mut self) -> crate::Result<()> {
        match self.head.read()? {
            Some(branch) => {
                let oid = self.refs.read(&branch)?;
                let commit: repository::Commit = self.db.retrieve(&oid)?;
                let root_tree: repository::Tree = self.db.retrieve(commit.root())?;
                let root = self.ws.path.clone();
                self.compare_tree_recursively(root_tree, &root)?;
            },
            None => {
                for file in self.ws.read_dir(&self.ws.path)? {
                    self.added(&file)?;
                }
            }
        }

        Ok(())
    }

    fn compare_tree_recursively(&mut self, tree: repository::Tree, path: &Path) -> crate::Result<()> {
        if !path.exists() {
            self.deleted(path)?;
            return Ok(());
        } else if !path.is_dir() {
            self.deleted(path)?;
            self.added(path)?;
            return Ok(());
        } 

        let mut visited = HashMap::new();
        let old = tree
            .entries()
            .into_iter()
            .map(|entry| {
                let mut path = path.to_path_buf();
                path.push(entry.name());
                let _ = visited.insert(path.clone(), false);
                (path, entry)
            })
            .collect::<HashMap<PathBuf, &repository::Entry>>();

        //todo: isn't file absolute path?
        for file in self.ws.read_dir(path)? {
            if let Some(entry) = old.get(&file) {
                let _ = visited.insert(file.clone(), true);
                if entry.is_dir() {
                    let sub_tree: repository::Tree = self.db.retrieve(entry.oid())?;
                    self.compare_tree_recursively(sub_tree, &file)?;
                } else if file.is_dir() {
                    self.deleted(&file)?;
                    self.added(&file)?;
                } else {
                    //db::entry and ws::entry both are file
                    self.compare_blob(entry, &file)?;
                }
            } else {
                self.added(&file)?;
            }
        }
        //todo: implement logic for deleted entries

        Ok(())
    }
    fn compare_blob(&mut self, entry: &repository::Entry, path: &Path) -> crate::Result<()> {
        let stat = self.ws.read_stat(path)?;
        if entry.mtime() != stat.mtime {
            self.modified(path)?;
        }
        Ok(())
    }
}
