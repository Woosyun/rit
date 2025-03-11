use crate::prelude::*;

pub struct Blob(String);

impl Blob {
    pub fn new(content: String) -> Self {
        Self(content)
    }
}

impl Objectify for Blob {
    fn objectify(&self) -> String {
        format!("blob {}", self.0)
    }
}
