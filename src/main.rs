mod core;
use crate::core::*;

mod inbuilt;
use crate::inbuilt::*;

use mlua::prelude::*;

// Helper function for resolving aliases.
fn resolve_alias(config: &mlua::Table, cmd: &str) -> Option<String> {
    config
        .get::<mlua::Table>("shellAliases")
        .ok()
        .and_then(|aliases| aliases.get(cmd).ok())
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
        display_prompt();

        // Get the users input.
        let input = get_input();

        // Tokenize the user input.
        let tokens = tokenize(&input);

        // If the user enters empty input skip to the next iteration.
        if tokens.is_empty() {
            continue;
        }

        // Convert tokens into command and arguments.
        let (mut cmd, mut args) = (get_command(&tokens), get_args(&tokens));

        // Check our configuration for aliases, convert alias to corresponding command and
        // arguments.
        if let Some(alias_cmd) = resolve_alias(&config, &cmd) {
            // Tokenize the alias command and update cmd and args accordingly.
            let alias_tokens = tokenize(&alias_cmd);
            cmd = get_command(&alias_tokens);
            args.extend(get_args(&alias_tokens));
        }

        // Execute inbuilt command if match found, otherwise execute external command with 
        // arguments.
        match cmd.as_str() {
            "cd" => cd(&args),
            "help" => help(),
            "exit" => exit(),
            _ => execute(&cmd, &args),
        }
    }
}
