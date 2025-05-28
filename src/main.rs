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
    // Remove newline character from the input. 
    input.pop();
    input
}

fn parse_input(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

fn get_command(tokens: Vec<&str>) -> &str {
    tokens[0]
}

fn get_args(tokens: Vec<&str>) -> Vec<&str> {
    tokens[1..].to_vec()
}

fn run_command(cmd: &str, args: Vec<&str>) {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("failed to execute process");

    assert!(status.success());
}

fn main() {
    loop {
        // Display the prompt for the user.
        display_prompt();

        // Get the users input.
        let input = get_input();

        // Parse the users input and create tokens.
        let tokens = parse_input(&input);

        // If the user enters empty input don't execute the command.
        if tokens.is_empty() {
            continue;
        }

        // Convert our tokens to our command and arguments.
        let cmd = get_command(tokens.clone());
        let args = get_args(tokens);

        // Run the command and arguments.
        run_command(cmd, args);
    }
}
