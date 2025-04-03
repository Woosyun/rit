use crate::repository::{Mtime, Oid};

pub trait Entry {
    fn mtime(&self) -> Mtime;
    fn oid(&self) -> Oid;
}

pub struct Revision {
    fn 
}
