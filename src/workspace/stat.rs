use filetime::FileTime;
use std::fs::Metadata;

pub struct Stat {
    pub mode: u32,
    // mtime represents seconds from unix epoch (is this agnostic?)
    pub mtime: i64,
}
impl Stat {
    pub fn from_metadata(metadata: Metadata) -> Self {
        let mode = if metadata.is_file() {
            if metadata.permissions().readonly() {
                Stat::readonly_file_mode()
            } else {
                Stat::executable_file_mode()
            }
        } else {
            Stat::directory_mode()
        };

        let mtime = FileTime::from_last_modification_time(&metadata)
            .unix_seconds();
        Self {
            mode,
            mtime,
        }
    }

    pub fn directory_mode() -> u32 {
        0o40000
    }
    pub fn readonly_file_mode() -> u32 {
        0o100644
    }
    pub fn executable_file_mode() -> u32 {
        0o100755
    }
}
