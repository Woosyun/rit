use crate::{
    repository::database::oid::Oid,
    workspace::Stat,
};
use serde::{Serialize, Deserialize};

type Mode = u32;
type Mtime = i64;
type Name = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry(pub Mode, pub Mtime, pub Oid, pub Name);
impl Entry {
    pub fn from_blob(stat: Stat, oid: Oid, name: String) -> Self {
        let mode = stat.mode;
        let mtime = stat.mtime;
        Self (
            mode,
            mtime,
            oid,
            name
        )
    }

    pub fn from_tree(oid: Oid, name: String) -> Self {
        let mode = Stat::directory_mode();
        let mtime = 0;
        Self (
            mode,
            mtime,
            oid,
            name
        )
    }
}
