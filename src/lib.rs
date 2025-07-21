pub mod workspace;
pub mod repository;
pub mod revision;
pub mod utils;
pub mod commands;

pub mod error;
pub use error::*;

#[cfg(test)]
pub mod test_utils;


pub mod prelude {
    pub use super::workspace::{
        self,
        Workspace,
        file::*,
        ignore::*,
        stat::*,
    };
    pub use super::repository::{
        self,
        Repository,
        database::{
            Database,
            Blob,
            Oid,
        },
        head::*,
        refs::*,
    };
    pub use super::revision::*;
    pub use super::error::*;
    pub use super::utils::*;
}
