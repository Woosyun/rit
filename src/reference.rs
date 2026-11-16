use crate::prelude::*;
use std::{io, path};

pub enum Reference {
    Oid(ObjectId),
    Sym(RelativePath),
}
impl Reference {
    pub fn is_oid(&self) -> bool {
        if let Self::Oid(_) = self { true } else { false }
    }
    pub fn is_symref(&self) -> bool {
        !self.is_oid()
    }

    pub fn parse(raw: Vec<u8>) -> io::Result<Self> {
        //tothink: What if length is 20 and starts with "ref: "
        let reference = if let Some(symref_raw) = raw.strip_prefix(b"refs: ") {
            let symref = String::from_utf8_lossy(symref_raw).into_owned();
            Reference::Sym(path::PathBuf::from(symref).try_into()?)
        } else {
            let oid = raw.try_into().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "cannot read ObjectId from this")
            })?;
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
