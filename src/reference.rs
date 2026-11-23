use crate::prelude::*;
use std::io;

pub enum Reference {
    Oid(ObjectId),
    Sym(RelativePath),
}
impl Reference {
    pub fn is_oid(&self) -> bool {
        matches!(self, Self::Oid(_))
    }
    pub fn is_symref(&self) -> bool {
        !self.is_oid()
    }

    pub fn parse(raw: Vec<u8>) -> io::Result<Self> {
        //tothink: What if length is 20 and starts with "ref: "
        let reference = if let Some(symref_raw) = raw.strip_prefix(b"ref: ") {
            let symref = String::from_utf8_lossy(symref_raw).into_owned();
            Reference::Sym(RelativePath::new(symref.into()))
        } else {
            let oid = <ObjectId as TryFrom<&[u8]>>::try_from(raw.as_ref())?;
            Reference::Oid(oid)
        };
        Ok(reference)
    }

    pub fn oid(&self) -> Option<&ObjectId> {
        match self {
            Reference::Oid(oid) => Some(oid),
            _ => None,
        }
    }

    pub fn relative_path(&self) -> Option<&RelativePath> {
        match self {
            Reference::Sym(relative_path) => Some(relative_path),
            _ => None,
        }
    }
}
