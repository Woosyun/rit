use sha2::{Digest, Sha256};
use std::io;

pub trait Objectify {
    fn objectify(&self) -> String;

    fn to_object(&self) -> (Oid, String) 
    where Self: Sized
    {
        let content = self.objectify();
        let oid = Oid::build(content.as_str());

        (oid, content)
    }
}

#[derive(Debug)]
pub struct Oid(Vec<u8>);
impl Oid {
    fn build(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize()
            .into_iter()
            .collect::<Vec<_>>();

        Self(result) 
    }

    pub fn split(&self) -> (String, String) {
        let mut hex = self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>();
        let file = hex.split_off(2).join("");
        let dir = hex.join("");

        (dir, file)
    }

    pub fn into_string(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn from_string(hex: String) -> io::Result<Self> {
        if hex.len() % 2 != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "len of input is not even"));
        }

        // todo: fix
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16).map_err(|e| e.to_string()))
            .collect::<Vec<_>>()
    }
}
