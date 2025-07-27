use std::{
    fs,
    io,
    path::Path,
    time::SystemTime,
};
use filetime::FileTime;

/*
pub fn get_file_name(path: &Path) -> crate::Result<String> {
    match path.file_name() {
        Some(file_name) => {
            let file_name = file_name
                .to_str().unwrap()
                .to_string();
            Ok(file_name)
        },
        None => {
            let f = format!("{:?}: cannot get file name. Maybe file name termiantes with ..", path);
            Err(crate::Error::Workspace(f))
        }
    }
}
*/

pub fn lock_write(file: &Path, content: &str) -> io::Result<()> {
    let mut lockfile = file.to_path_buf();
    lockfile.set_extension("lock");

    fs::write(&lockfile, content)?;
    fs::rename(&lockfile, file)?;

    Ok(())
}

pub fn set_file_mtime(path: &Path, mtime: i64) -> io::Result<()> {
    let file_time = FileTime::from_unix_time(mtime, 0);
    filetime::set_file_mtime(path, file_time)
}

pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("cannot get time")
        .as_secs()
}
