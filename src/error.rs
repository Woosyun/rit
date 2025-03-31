#[derive(Debug)]
pub enum Error {
    Io(String),
    Workspace(String),
    Repository(String),
    Unknown(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Io(s) => write!(f, "(io){}", s),
            Error::Workspace(s) => write!(f, "(workspace){}", s),
            Error::Repository(s) => write!(f, "(repository){}", s),
            Error::Unknown(s) => write!(f, "(unknown){}", s),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
