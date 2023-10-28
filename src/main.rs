use json;
use std::io;

fn main() {
    loop {
        let mut args = String::new();
        io::stdin()
            .read_line(&mut args)
            .expect("Could not read user input!");

        let args = get_args(args);

        match args[0].as_str() {
            "exit" => break,
            "help" => print_help(),
            "h" => print_help(),
            _ => print_help(),
        }
    }
}

fn print_help() {
    println!("No help here!");
}

fn get_args(args: String) -> Vec<String> {
    let mut split_args: Vec<String> = Vec::new();
    let split_string = args.split_whitespace();
    for arg in split_string {
        split_args.push(String::from(arg));
    }

    return split_args;
}
