use sha1::{Digest, Sha1};
use std::error;

pub struct Oid([u8; 20]);

impl Oid {
    pub fn new(content: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut oid = [0u8; 20];
        let hash = Sha1::digest(content);
        oid.copy_from_slice(hash.as_slice());

        Ok(Self ( oid ))
    }
    pub fn dir_name(&self) -> String {
        self.0[0..2]
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
    pub fn file_name(&self) -> String {
        self.0[2..]
            .into_iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}
