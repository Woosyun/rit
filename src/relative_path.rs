use std::path;

pub struct RelativePath(path::PathBuf);
impl From<path::PathBuf> for RelativePath {
    fn from(item: path::PathBuf) -> Self {
        Self(item)
    }
}
impl From<RelativePath> for path::PathBuf {
    fn from(item: RelativePath) -> path::PathBuf {
        item.0
    }
}
