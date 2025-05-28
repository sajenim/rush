use std::io::{self, Write};
use std::process::Command;

fn display_prompt() {
    print!("> ");
    io::stdout().flush().expect("unable to flush buffer");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    input.pop();
    return input;
}

fn parse_input(s: &str) -> Vec<&str> {
    return s.split_whitespace().collect();
}

fn get_command(v: Vec<&str>) -> String {
    return v[0..1].concat();
}

fn get_args(v: Vec<&str>) -> Vec<&str> {
    return v[1..].to_vec();
}

fn run_command(cmd: String, args: Vec<&str>) {
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()
        .expect("failed to execute process");

    child.wait().expect("failed to wait on child");
}

fn main() {
    loop {
        // Display the prompt for the user and wait for input.
        display_prompt();

        // Get the users input and store it as &str
        let input = get_input();
        let input = input.as_str();

        // Parse the users input and create tokens.
        let tokens = parse_input(input);

        // Convert our tokens to our command and arguments.
        let cmd = get_command(tokens.clone());
        let args = get_args(tokens.clone());

        // Run the command and arguments.
        run_command(cmd, args);
    }
}
