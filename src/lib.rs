// commands
pub mod commit;
pub mod init;

// core components
pub mod index;
pub mod object;
pub mod reference;
pub mod repository;

// utilities
pub mod lockfile;
// pub mod relative_path;
pub mod sha1;
pub mod zlib;

pub mod prelude {
    pub use super::commit;
    pub use super::init;

    pub use super::index::*;
    pub use super::object::*;
    pub use super::reference::*;
    pub use super::repository::*;

    pub use super::lockfile::*;
    // pub use super::relative_path::*;
    pub type RelativePath = std::path::PathBuf;
    pub use super::sha1::*;
    pub use super::zlib::*;
}
