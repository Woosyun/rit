pub mod workspace;
pub mod repository;
pub mod revision;
pub mod utils;
pub mod commands;

pub mod fs;

pub mod error;
pub use error::*;

pub mod prelude {
    pub use super::*;

    pub use workspace::{
        self,
        Workspace,
        Ignore,
        File,
        stat::{
            Stat,
            Mode,
            Mtime,
            Name,
        },
    };
    pub use repository::{
        self,
        Repository,
        Blob,
        Oid,
    };
    pub use revision::{
        Revision,
        IntoRev,
        Rev,
        RevDiff,
    };
}
