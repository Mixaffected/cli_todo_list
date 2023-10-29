use std::io;
mod todo_file_manager;

fn main() {
    let file_manager = todo_file_manager::TodoFileManger {
        path: "todo_list.sav".to_string(),
    };
    loop {
        print!(">>> ");
        let result = io::Write::flush(&mut io::stdout());
        match result {
            Ok(_) => {}
            Err(_) => {
                println!("Could not flush write buffer!");
                continue;
            }
        }

        let mut args = String::new();
        io::stdin()
            .read_line(&mut args)
            .expect("Could not read user input!");

        let args = get_args(args);

        match args[0].as_str() {
            "show" => show_todo_entries(&file_manager),
            "add" => add_todo_entry(args, &file_manager),
            "exit" => break,
            "help" => print_help(),
            "h" => print_help(),
            _ => print_help(),
        }
    }
}

fn show_todo_entries(file_manager: &todo_file_manager::TodoFileManger) {
    let todo_list = file_manager.read_file();
    let todo_list = match todo_list {
        Ok(todo_list) => todo_list,
        Err(_) => return println!("Something went wrong by reading file!"),
    };

    for (i, line) in todo_list.lines().enumerate() {
        println!("{}: {}", i, line);
    }
}

fn add_todo_entry(args: Vec<String>, file_manager: &todo_file_manager::TodoFileManger) {
    let mut todo_entry: String = String::new();
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let mut out_arg: String = arg.clone();
        out_arg.push(' ');
        todo_entry.push_str(out_arg.as_str());
    }

    let todo_list_content = file_manager.read_file();
    let mut todo_list_content = match todo_list_content {
        Ok(todo_list_content) => todo_list_content,
        Err(_) => return println!("Could not read file!"),
    };

    if todo_list_content.lines().count() > 0 {
        todo_entry.insert_str(0, "\n");
    }
    todo_list_content.push_str(&todo_entry);
    let result = file_manager.write_file(todo_list_content);
    match result {
        Ok(_) => return,
        Err(_) => return println!("Could not add task to list!"),
    }
}

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
