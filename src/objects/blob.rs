use crate::prelude::*;

pub struct Blob {
    oid: Option<Hash256>,
    content: String,
}
impl Blob {
    pub fn new(content: String) -> Self {
        Self {
            oid: None,
            content
        }
    }
}
impl Objectify for Blob {
    fn objectify(&self) -> String {
        self.content.clone()
    }
    fn set_oid(&mut self, oid: Hash256) {
        self.oid = Some(oid);
    }
    fn get_oid(&self) -> Option<&Hash256> {
        self.oid.as_ref()
    }
}
