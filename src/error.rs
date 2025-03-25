#[derive(Debug)]
pub enum Error {
    Io(String),
    Workspace(String),
    Repository(String),
    Unknown(String),
}
pub type Result<T> = std::result::Result<T, Error>;
