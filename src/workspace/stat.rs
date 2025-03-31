use filetime::FileTime;
use std::fs::Metadata;
use serde::{Serialize, Deserialize};
use crate::repository::{Mtime, Mode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub mode: Mode,
    // mtime represents seconds from unix epoch (is this os agnostic?)
    pub mtime: Mtime,
}
impl Stat {
    pub fn from_metadata(metadata: &Metadata) -> Self {
        let mode = if metadata.is_file() {
            if metadata.permissions().readonly() {
                Stat::readonly_file_mode()
            } else {
                Stat::executable_file_mode()
            }
        } else {
            Stat::directory_mode()
        };

        let mtime = FileTime::from_last_modification_time(metadata)
            .unix_seconds();
        Self {
            mode,
            mtime,
        }
    }

    pub fn directory_mode() -> Mode {
        0o40000
    }
    pub fn readonly_file_mode() -> Mode {
        0o100644
    }
    pub fn executable_file_mode() -> Mode {
        0o100755
    }
    pub fn is_dir(&self) -> bool {
        if self.mode == Stat::directory_mode() {
            true
        } else {
            false
        }
    }
}
