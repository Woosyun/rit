use crate::prelude::*;
use std::{fs, io, path};

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
    fn parse(raw: Vec<u8>) -> io::Result<Self> {
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

pub struct References {
    path: path::PathBuf,
}
impl References {
    pub fn new(path: path::PathBuf) -> Self {
        Self { path }
    }

    pub fn read(&self, relative_path: &RelativePath) -> io::Result<Reference> {
        let target_path: path::PathBuf = self.path.join(relative_path);
        let target_raw = fs::read(target_path)?;
        Reference::parse(target_raw)
    }
    pub fn write(&self, relative_path: &RelativePath, oid: ObjectId) -> io::Result<()> {
        let target_path = self.path.join(relative_path);
        fs::write(target_path, oid)
    }
}
