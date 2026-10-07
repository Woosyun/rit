use super::IntoObject;

const BLOB_TYPE: &'static str = "blob";

pub struct Blob(String);
impl Blob {
    pub fn new(content: String) -> Self {
        Blob(content)
    }
}

impl IntoObject for Blob {
    fn into_object(self) -> String {
        format!("{} {}\0{}", BLOB_TYPE, self.0.len(), self.0)
    }
}
