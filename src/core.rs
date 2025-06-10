use std::io::{self, Write};
use std::process::Command;

pub fn display_prompt(ps1: &str) {
    print!("{}", ps1);
    io::stdout()
        .flush()
        .expect("unable to flush buffer");
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

pub fn tokenize(s: &str) -> Vec<String> {
    shlex::split(s).unwrap()
}

pub fn get_command(tokens: &[String]) -> String {
    tokens[0].clone()
}

pub fn get_args(tokens: &[String]) -> Vec<String> {
    tokens[1..].to_vec()
}

pub fn execute(cmd: &str, args: &[String]) {
    let status = Command::new(cmd)
        .args(args)
        .status();

    if status.is_err() {
        println!("{}: command not found", cmd)
    }
}
