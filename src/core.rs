use std::io::{self, Write};
use std::process::Command;

pub fn display_prompt() {
    print!("> ");
    io::stdout().flush().expect("unable to flush buffer");
}

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    // Remove newline character from the input.
    input.pop();
    input
}

pub fn parse_input(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn get_command(tokens: Vec<&str>) -> &str {
    tokens[0]
}

pub fn get_args(tokens: Vec<&str>) -> Vec<&str> {
    tokens[1..].to_vec()
}

pub fn execute(cmd: &str, args: Vec<&str>) {
    let status = Command::new(cmd)
        .args(args)
        .status();

    if status.is_err() {
        println!("{}: command not found", cmd)
    }
}
