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

// 전부 crate로 만들어서 테스트 코드를 작성하게 되면
// 테스트의 완전성은 테스트 코드의 완성도에 비례하고
// command의 알고리즘은 검사할 수가 없게 된다.
// 그냥 코드를 잘 짜보면 되지 않을까?
