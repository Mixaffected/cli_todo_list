use json;
use std::io;

fn main() {
    loop {
        let mut args = String::new();
        io::stdin()
            .read_line(&mut args)
            .expect("Could not read user input!");

        let args = get_args(args);

        for arg in &args {
            println!("{}", arg);
        }

        match args[0].as_str() {
            "exit" => break,
            "help" => print_help(),
            "h" => print_help(),
            _ => print_help(),
        }
    }
}

fn show_todo_entries(args: Vec<String>) {}

fn add_todo_entry(args: Vec<String>) {}

fn remove_todo_entry(args: Vec<String>) {}

fn print_help() {
    println!("No help here!");
}

// convert string to args
fn get_args(string_args: String) -> Vec<String> {
    let mut split_args: Vec<String> = Vec::new();

    let mut current_part: String = String::new();
    let mut is_string: bool = false;
    for arg_char in string_args.trim().chars() {
        if arg_char == '"' {
            is_string = !is_string;
            continue;
        }

        if is_string {
            current_part.push(arg_char);
            continue;
        } else if arg_char == ' ' {
            split_args.push(current_part.clone());
            current_part = String::new();
            continue;
        }

        current_part.push(arg_char);
    }

    split_args.push(current_part);

    return split_args;
}
