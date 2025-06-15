use std::{
    time::Duration,
    thread,
    path::Path,
    io::{self, Write},
    fs::*,
};

pub fn sleep_1_sec() {
    thread::sleep(Duration::from_secs(1));
}
pub fn appendln(path: &Path, content: &str) -> io::Result<()> {
    let mut fd = OpenOptions::new().append(true).open(path)?;
    writeln!(fd, "{}", content)
}
