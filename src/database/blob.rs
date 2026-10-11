use super::IntoObject;

pub const BLOB_TYPE: &'static str = "blob";

pub struct Blob(Vec<u8>);
impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        /*
        let bytes = format!("{} {}\0", BLOB_TYPE, content.len()).into_bytes();
        Blob([bytes, content].concat())
*/
        Blob ( content )
    }
}

impl IntoObject for Blob {
    fn into_object(self) -> Vec<u8> {
        let header = format!("{} {}\0", BLOB_TYPE, self.0.len()).into_bytes();
        [header, self.0].concat()
    }
}
