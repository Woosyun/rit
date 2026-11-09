use super::Object;
use std::error;

pub const BLOB_TYPE: &'static str = "blob";

pub struct Blob(Vec<u8>);
impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        /*
                let bytes = format!("{} {}\0", BLOB_TYPE, content.len()).into_bytes();
                Blob([bytes, content].concat())
        */
        Blob(content)
    }
}

impl From<Blob> for Object {
    fn from(blob: Blob) -> Object {
        let header = format!("{} {}\0", BLOB_TYPE, blob.0.len()).into_bytes();
        [header, blob.0].concat()
    }
}

impl TryFrom<Object> for Blob {
    type Error = Box<dyn error::Error>;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        todo!()
    }
}
