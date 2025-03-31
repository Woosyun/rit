use std::{
    path::PathBuf,
    fs,
    io,
};

pub fn run(cwd: PathBuf) -> io::Result<()> {
    let mut temp_file1 = cwd.clone();
    temp_file1.push("hello.txt");
    fs::write(&temp_file1, "hello")
        .expect("could not write to hello.txt");

    let mut temp_file2 = cwd;
    temp_file2.push("hello");
    fs::create_dir(&temp_file2)
        .expect("could not create hello folder in the temporary directory");
    temp_file2.push("world.txt");
    fs::write(&temp_file2, "world")
        .expect("could not write to hello/world.txt");
    Ok(())
}
