use std::{
    path::PathBuf,
    fs,
    io,
};

pub fn work1(cwd: PathBuf) -> io::Result<()> {
    let mut temp_file1 = cwd.clone();
    temp_file1.push("hello.txt");
    fs::write(&temp_file1, "hello").unwrap();

    let mut temp_file2 = cwd;
    temp_file2.push("hello");
    fs::create_dir(&temp_file2).unwrap();
    temp_file2.push("world.txt");
    fs::write(&temp_file2, "world").unwrap();

    Ok(())
}

pub fn work2(cwd: PathBuf) -> io::Result<()> {
    let mut temp_file1 = cwd.clone();
    temp_file1.push("new.txt");
    fs::write(&temp_file1, "new").unwrap();

    let mut temp_file2 = cwd.clone();
    temp_file2.push("hello.txt");
    fs::write(&temp_file2, "modified").unwrap();

    let mut temp_file3 = cwd.clone();
    temp_file3.push("hello");
    temp_file3.push("world.txt");
    fs::remove_file(&temp_file3).unwrap();

    Ok(())
}
