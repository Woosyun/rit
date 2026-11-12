use super::{Object, ObjectKind};

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
        Object::new(ObjectKind::Blob, blob.0.len(), blob.0)
    }
}
