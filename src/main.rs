use rit::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("command is missing");
    }

    let cwd = std::env::current_dir()
        .expect("cannot get current directory");
    let command = Command::build(cwd).expect("cannot build Command object");

    let cmd = args[1].clone();
    let result = match cmd.as_str() {
        "commit" => {
            if args.len() != 3 {
                println!("commit message is missing");
                return;
            }

            let message = args[2].clone();
            command.commit(message)
                .expect("cannot execute commit")
        },
        "init" => {
            command.init()
                .expect("cannot run init")
        },
        "status" => {
            command.status()
                .expect("cannot run status")
        },
        _ => {
            "Unsupported command".to_string()
        }
    };

    println!("result: \n{}", result);
}
