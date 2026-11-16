use super::{Object, ObjectKind};

pub struct Blob {
    data: Vec<u8>,
}
impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        Blob { data }
    }
}

impl From<Blob> for Object {
    fn from(blob: Blob) -> Object {
        Object::new(ObjectKind::Blob, blob.data.len(), blob.data)
    }
}
