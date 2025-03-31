use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Self (content)
    }
}
