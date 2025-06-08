mod core;
mod inbuilt;

fn main() {
    loop {
        // Display the prompt for the user.
        core::display_prompt();

        // Get the users input.
        let input = core::get_input();

        // Parse the users input and create tokens.
        let tokens = core::parse_input(&input);

        // If the user enters empty input don't execute the command.
        if tokens.is_empty() {
            continue;
        }

        // Convert our tokens to our command and arguments.
        let cmd = core::get_command(&tokens);
        let args = core::get_args(&tokens);

        // Execute inbuilt commands if supplied, otherwise execute command and argument.
        match cmd.as_str() {
            "cd" => inbuilt::cd(&args),
            "help" => inbuilt::help(),
            "exit" => inbuilt::exit(),
            _ => core::execute(&cmd, &args),
        }
    }
}
