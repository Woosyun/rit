use super::{Object, ObjectId, ObjectKind};

pub const EXECUTABLE_FILE_MODE: Mode = 0o100755;
pub const READONLY_FILE_MODE: Mode = 0o100644;
pub const DIRECTORY_MODE: Mode = 0o040000;

/// Mode would be stored as 6-bytes
type Mode = u32;
pub type TreeEntry = (Mode, ObjectId, String);

pub struct Tree {
    entries: Vec<TreeEntry>,
}
impl Tree {
    pub fn new(entries: Vec<TreeEntry>) -> Self {
        Tree { entries }
    }
}

impl From<Tree> for Object {
    fn from(tree: Tree) -> Object {
        //sort
        let mut entries = tree.entries;
        entries.sort_by(|from, to| from.2.cmp(&to.2));

        //SPACE, NULL(\0)?
        let data = entries
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
        Object::new(ObjectKind::Tree, data.len(), data)
    }
}
