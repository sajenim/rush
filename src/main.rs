mod core;
mod inbuilt;

use mlua::prelude::*;

// Helper function for resolving aliases.
fn resolve_alias(config: &mlua::Table, cmd: &str) -> Option<String> {
    config
        .get::<mlua::Table>("aliases")
        .ok()
        .and_then(|aliases| aliases.get::<String>(cmd).ok())
}

fn main() {
    // Create a new Lua state and load the safe subset of the standard libraries.
    let lua = Lua::new();

    // Retreive Lua source code from configuration file.
    lua.load("config = require 'rush'")
        .exec()
        .expect("Failed to load configuration.");

    // Create configuration table.
    let config: mlua::Table = lua
        .globals()
        .get("config")
        .expect("Failed to create configuration table.");

    // Read, Evaluate, Print and Loop.
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

        // Convert tokens into command and arguments.
        let (mut cmd, mut args) = (core::get_command(&tokens), core::get_args(&tokens));

        // Check our configuration for aliases, convert alias to corresponding command.
        if let Some(alias_cmd) = resolve_alias(&config, &cmd) {
            // Tokenize the alias command and update cmd and args accordingly.
            let alias_tokens = core::parse_input(&alias_cmd);
            cmd = core::get_command(&alias_tokens);
            args.extend(core::get_args(&alias_tokens));
        }

        // Execute inbuilt commands if supplied, otherwise execute command and argument.
        match cmd.as_str() {
            "cd" => inbuilt::cd(&args),
            "help" => inbuilt::help(),
            "exit" => inbuilt::exit(),
            _ => core::execute(&cmd, &args),
        }
    }
}
