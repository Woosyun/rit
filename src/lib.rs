// module for commands
pub mod commands;

// core data
pub mod index;
pub mod object;

// frontend?
pub mod repository;
pub mod workspace;

// utility stuff
pub mod command;
pub mod lockfile;
pub mod relative_path;
pub mod sha1;
pub mod zlib;

// prelude for dev
pub mod prelude {
    //1
    pub use super::index::*;
    pub use super::object::*;

    //2
    pub use super::repository::*;
    pub use super::workspace::*;

    //3
    pub use super::command::*;
    pub use super::lockfile::*;
    pub use super::relative_path::*;
    pub use super::sha1;
    pub use super::zlib;
}
