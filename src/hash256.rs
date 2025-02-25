use std::hash::Hash;
use crate::traits::*;

#[derive(Clone, Hash)]
pub struct Hash256(Vec<u8>);

impl Hash256 {
    pub fn build(content: Vec<u8>) -> Self {
        Self ( content )
    }

    pub fn split(&self) -> (String, String) {
        let dir = self.0[0..2]
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join("");
        let file = self.0[2..]
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join("");

        (dir, file)
    }
}

impl Objectify for Hash256 {
    fn objectify(&self) -> String {
        self.0
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}
