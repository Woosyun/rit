use crate::{
    workspace::Stat,
    repository::database::oid::Oid,
};

pub struct Entry {
    pub stat: Stat,
    pub name: String,
    pub oid: Oid,
}
impl Entry {
    /*
    pub fn from_blob(blob: workspace::Blob) -> Result<Self, &'static str> {
    }
*/
}
