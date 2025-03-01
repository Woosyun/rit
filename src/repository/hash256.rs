use std::{
    hash::Hash,
    io
};

#[derive(Clone, Hash)]
pub struct Hash256(Vec<u8>);

impl Hash256 {
    pub fn new(content: Vec<u8>) -> io::Result<Self> {
        if content.len() != 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "hash value must has 32 bytes"));
        }

        Ok(Self(content))
    }
    pub fn split(&self) -> (String, String) {
        let mut hx = self.0
            .iter()
            .map(|b| format!("{:x?}", b))
            .collect::<Vec<_>>();

        let dir = hx.split_off(2)
            .join("");
        let file = hx.join("");

        (dir, file)
    }
}
impl From<Hash256> for String {
    fn from(item: Hash256) -> String {
        item.0
            .iter()
            .map(|u| u.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}
