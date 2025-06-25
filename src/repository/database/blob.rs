use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Self (content)
    }
}

impl std::ops::Deref for Blob {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl AsRef<[u8]> for Blob {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
