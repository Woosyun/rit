use std::{
    path::PathBuf,
    collections::{HashMap, HashSet},
};

type Index = PathBuf; // index represents path relative to work directory

pub struct WorkspaceGhost {
    entries: HashMap<Index, EntryGhost>,
    repository: RepositoryGhost,
}
impl WorkspaceGhost {
    pub fn new() -> Self {
        Self {
            entries: HashSet::new(),
            repository: RepositoryGhost::new(),
        }
    }
}
impl GhostDiff for WorkspaceGhost {
    fn ghost_diff(&self, another: Workspace) -> Result<GhostDiff> {
        //compare entries
        let ghost = self.into_rev()?;
        let real = another.into_rev()?;
        for (index, entry_ghost) in ghost {
            entry_ghost.diff(read.get(index))?;
        }
        //compare repository
    }
}
enum EntryGhost {
    File(FileGhost),
    Directory(DirectoryGhost),
}
struct FileGhost {
    path: PathBuf,
    content: String,
    mtime: u64,
}
struct DirectoryGhost {
    path: PathBuf,
    entries: HashMap<Index, EntryGhost>,
}

struct RepositoryGhost {
    local_head: LocalHeadGhost,
    database: HashMap<Oid, ObjectGhost>,
    refs: RefsGhost,
}
enum LocalHeadGhost {
    Oid(Oid),
    Brach(String),
}
enum ObjectsGhost {
    Blob(FileGhost),
    Tree(DirectoryGhost),
    Commit(CommitGhost),
}
struct RefsGhost {
    local_branches: HashMap<String, Oid>,
}
