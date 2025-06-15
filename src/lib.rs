pub mod workspace;
pub mod repository;
pub mod revision;
pub mod utils;
pub mod commands;

pub mod error;
pub use error::*;

pub mod prelude {
    pub use super::workspace::{
        self,
        Workspace,
        Ignore,
        File,
        stat::*,
    };
    pub use super::repository::{
        self,
        Repository,
        Database,
        head::{Head, LocalHead},
        refs::Refs,
        Blob,
        Oid,
    };
    pub use super::revision::{
        Revision,
        IntoRev,
        Rev,
        RevDiff,
    };
    pub use super::commands;
    pub use super::error::*;
    pub use super::utils::{self, *};
}
