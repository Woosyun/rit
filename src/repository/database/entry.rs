use crate::{
    repository::Oid,
    workspace::Stat,
};
use serde::{Serialize, Deserialize};

pub type Mode = u32;
pub type Mtime = i64;
type Name = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entry(pub Mode, pub Mtime, pub Oid, pub Name);
impl Entry {
    pub fn mode(&self) -> Mode {
        self.0
    }
    pub fn mtime(&self) -> Mtime {
        self.1
    }
    pub fn oid(&self) -> &Oid {
        &self.2
    }
    pub fn name(&self) -> &Name {
        &self.3
    }
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
    pub fn is_dir(&self) -> bool {
        if self.mode() == Stat::directory_mode() {
            true
        } else {
            false
        }
    }
}
