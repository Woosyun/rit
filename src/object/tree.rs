use super::{Object, ObjectId};

pub const EXECUTABLE_FILE_MODE: Mode = 0o100755;
pub const READONLY_FILE_MODE: Mode = 0o100644;
pub const DIRECTORY_MODE: Mode = 0o040000;

/// Mode would be stored as 6-bytes
type Mode = u32;
type EntryName = String;
pub type Entry = (Mode, ObjectId, EntryName);

pub struct Tree {
    entries: Vec<Entry>,
}
impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Tree { entries }
    }
}

impl From<Tree> for Object {
    fn from(tree: Tree) -> Object {
        //sort
        let mut entries = tree.entries;
        entries.sort_by(|from, to| from.2.cmp(&to.2));

        //SPACE, NULL(\0)?
        //todo: add oid for tree
        let content = entries
            .into_iter()
            .map(|(mode, oid, name)| {
                [
                    format!("{mode} ").into_bytes(),
                    format!("{name}\0").into_bytes(),
                    oid.to_vec(),
                ]
                .concat()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat();
        Object::new(content)
    }
}
