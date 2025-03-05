use sha2::{Digest, Sha256};

pub trait Objectify {
    fn objectify(&self) -> String;

    fn calculate_hash(&self) -> Hash256 {
        let mut hasher = Sha256::new();
        hasher.update(self.objectify());

        let hash = hasher.finalize()
            .into_iter()
            .collect::<Vec<_>>();

        Hash256::new(hash)
    }
}

#[derive(Clone)]
pub struct Hash256(Vec<u8>);

impl Hash256 {
    pub fn new(content: Vec<u8>) -> Self {
        Self(content)
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
