pub const EXECUTABLE_FILE_MODE: Mode = 0o100755;
pub const READONLY_FILE_MODE: Mode = 0o100644;
pub const DIRECTORY_MODE: Mode = 0o040000;

/// Mode would be stored as 6-bytes
type Mode = u32;
type Oid = super::Oid;
type EntryName = String;
pub type Entry = (Mode, Oid, EntryName);

pub struct Tree {
    entries: Vec<Entry>,
}
impl Tree {
    /*
    pub fn new() -> Self {
        Tree { entries: vec![] }
    }
    */
    pub fn from(entries: Vec<Entry>) -> Self {
        Tree { entries }
    }

    /*
    pub fn insert(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
    */
}

impl super::IntoObject for Tree {
    fn into_object(self) -> Vec<u8> {
        //sort
        let mut entries = self.entries;
        entries.sort_by(|from, to| from.2.cmp(&to.2));

        //SPACE, NULL(\0)?
        //todo: add oid for tree
        entries
            .into_iter()
            .map(|(mode, oid, name)| {
                [
                    format!("{mode} ").into_bytes(),
                    format!("{name}\0").into_bytes(),
                    oid.0.to_vec(),
                ]
                .concat()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
}
