use std::fmt::Arguments;
use std::io::{self, Write};

static HELP_MESSAGE: &str = "Available commands:
- help: Show this help message
- exit, quit: Exit the shell";

pub fn main_repl_loop() {
    let mut running = true;

    while running {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "exit" | "quit" => {
                running = false;
                break;
            }
            command => {
                match execute_command(command) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        };
    }
}

fn execute_command(command: &str) -> Result<String, String> {
    let command = command.split_whitespace().collect::<Vec<_>>();
    match command[0] {
        "help" => Ok(HELP_MESSAGE.to_string()),
        "new" => new_command(&command[1..]),
        _ => Err(format!("Unknown command: {}", command[0])),
    }
}

fn new_command(arguments : Vec<String>) -> Result<String, String> {
    if arguments.len() != 3 {
        return Err(format!("new command requires exactly 3 arguments, got {}", arguments.len()));
    }
    match arguments
}
