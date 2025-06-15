use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    Workspace(String),
    Ignore(String),
    Repository(String),
    Database(String),
    Refs(String),
    LocalHead(String),
    Commands(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Workspace(s) => write!(f, "(workspace){}", s),
            Error::Ignore(s) => write!(f, "(ignore){}", s),
            Error::Repository(s) => write!(f, "(repository){}", s),
            Error::Database(s) => write!(f, "(database){}", s),
            Error::Refs(s) => write!(f, "(refs){}", s),
            Error::LocalHead(s) => write!(f, "(local head){}", s),
            Error::Commands(s) => write!(f, "(unknown){}", s),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
