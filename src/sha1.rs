use sha1::{Digest, Sha1};

pub fn hash<U: AsRef<[u8]>>(obj: U) -> [u8; 20] {
    let mut oid = [0u8; 20];
    let hash = Sha1::digest(obj);
    oid.copy_from_slice(hash.as_slice());
    oid
}
