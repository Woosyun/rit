use sha2::{Digest, Sha256};
use std::{
    io,
    fmt::Write,
};

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

    pub fn decode(&self) -> String {
        let mut s = String::with_capacity(self.0.len() * 2);
        for b in &self.0 {
            write!(s, "{:02x}", b).unwrap();
        }
        s
    }

    pub fn encode(hex: &str) -> io::Result<Self> {
        if hex.chars().count() != 64 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "hex string should be 64 characters"));
        }

        let mut result = Vec::with_capacity(hex.len() / 2);
        for i in (0..hex.len()).step_by(2) {
            let b = u8::from_str_radix(&hex[i..i+2], 16)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "cannot parse hex"))?;
            result[i/2] = b;
        }


        Ok(Oid(result))
    }
}
