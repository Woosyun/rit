#![allow(unused)]
const EXECUTABLE_MODE: u16 = 0o100755;
const READONLY_MODE: u16 = 0o100644;
//const DIR: u16 = 0o040000

type Mode = String;
type Oid = super::Oid;
type Name = String;
type Entry = (Mode, Oid, Name);

pub struct Tree {
    entries: Vec<Entry>,
}
impl Tree {
    pub fn new() -> Self {
        Tree {
            entries: vec![]
        }
    }
    pub fn from(entries: Vec<Entry>) -> Self {
        Tree {
            entries
        }
    }

    pub fn insert(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl super::IntoObject for Tree {
    fn into_object(self) -> Vec<u8> {
        //sort
        let mut entries = self.entries;
        entries.sort_by(|from, to| from.2.cmp(&to.2));

        //SPACE, NULL(\0) can be better to handle if seperated?
        entries.into_iter()
            .map(|(mode, oid, name)| {
                [
                    format!("{mode} ").into_bytes(),
                    format!("{name}\0").into_bytes()
                    oid.0.to_vec(),
                    //todo: add oid
                ].concat()
            })
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
}
