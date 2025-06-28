use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Oid(String);
impl Oid {
    pub fn build(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = hasher.finalize();

        let mut hex = String::with_capacity(64);
        for byte in hash {
            use std::fmt::Write;
            write!(&mut hex, "{:02x}", byte).unwrap();
        }

        debug_assert_eq!(hex.len(), 64);

        Self(hex) 
    }

    pub fn split(&self) -> (&str, &str) {
        self.0.split_at(4)
    }
}

impl std::ops::Deref for Oid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::borrow::Borrow<str> for Oid {
    fn borrow(&self) -> &str {
        &self.0
    }
}
