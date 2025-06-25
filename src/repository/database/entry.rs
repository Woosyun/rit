use crate::{
    repository::Oid,
    workspace::stat::*,
};
use serde::{Serialize, Deserialize};

type Name = String;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Entry(pub Mode, pub Mtime, pub Oid, pub Name);
impl Entry {
    pub fn new(mode: Mode, mtime: Mtime, oid: Oid, name: Name) -> Self {
        Self(mode, mtime, oid, name)
    }
    pub fn build(stat: &dyn Stat) -> crate::Result<Self> {
        let oid = stat.oid()?.clone();
        let name = stat.name().to_string();
        let result = Self(stat.mode(), stat.mtime(), oid, name);
        Ok(result)
    }
}

impl Stat for Entry {
    fn mode(&self) -> Mode {
        self.0
    }
    fn mtime(&self) -> Mtime {
        self.1
    }
    fn oid(&self) -> crate::Result<&Oid> {
        Ok(&self.2)
    }
    fn set_oid(&mut self, _: Oid) {
        ()
    }
    fn name(&self) -> &Name {
        &self.3
    }
}
