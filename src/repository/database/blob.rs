use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Self (content)
    }
}
