use crate::prelude::*;

#[derive(std::hash::Hash)]
pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Self(content)
    }
}
impl Objectify for Blob {
    fn objectify(&self) -> String {
        self.0.clone()
    }
}
impl CalculateHash for Blob {}

