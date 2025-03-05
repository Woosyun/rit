use crate::prelude::*;
use std::io;

#[derive(std::hash::Hash)]
pub struct Blob {
    pub name: String,
    pub oid: Option<Hash256>,
    pub content: String,
}
impl Blob {
    pub fn new(name: String, content: String) -> Self {
        Self {
            name,
            oid: None,
            content,
        }
    }

    pub fn set_oid(&mut self) -> io::Result<()> {
        let oid = self.calculate_hash()?;
        self.oid = Some(oid);
        Ok(())
    }
}
impl Objectify for Blob {
    fn objectify(&self) -> String {
        self.content.clone()
    }
}
impl CalculateHash for Blob {}
impl From<Blob> for Entry {
    fn from(item: Blob) -> Entry {
        Entry::new(EntryKind::Blob, item.oid.unwrap(), item.name)
    }
}
