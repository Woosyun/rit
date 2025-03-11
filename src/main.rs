use rit::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("command is missing");
    }

    let cwd = std::env::current_dir()
        .expect("cannot get current directory");
    let result = match args[1].as_str() {
        "commit" => {
            let command = Command::build(cwd).expect("cannot build Command object");

            if args.len() != 3 {
                println!("commit message is missing");
                return;
            }

            let message = args[2].clone();
            command.commit(message)
                .expect("cannot execute commit")
        },
        "init" => {
            let result = Command::init(cwd)
                .expect("cannot init");
            result.to_string()
        },
        _ => {
            "Unsupported command".to_string()
        }
    };

    println!("result: \n{}", result);
}
