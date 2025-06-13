mod core;
use core::*;

mod inbuilt;
use inbuilt::*;

use mlua::prelude::*;

// Helper function for resolving aliases.
fn resolve_alias(config: &mlua::Table, cmd: &str) -> Option<String> {
    config
        .get::<mlua::Table>("shellAliases")
        .ok()
        .and_then(|aliases| aliases.get(cmd).ok())
}

// Return prompt from configuration file with expansion performed.
// Otherwise return default prompt.
fn resolve_prompt(config: &mlua::Table) -> String {
    let default_prompt = "[${user}@${host}:${cwd}]$".to_string();

    let prompt = config
        .get::<String>("prompt")
        .unwrap_or(default_prompt)
        + " ";

    shellexpand::env_with_context_no_errors(&prompt, context).to_string()
}

// Format current working directory.
fn fmt_cwd() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let cwd = std::env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or_default();

    // Replace $HOME with ~.
    if cwd.starts_with(&home) {
        cwd.replacen(&home, "~", 1)
    } else {
        cwd
    }
}

// Context for shell expansion.
fn context(s: &str) -> Option<String> {
    match s {
        "user" => Some(whoami::username()),
        "host" => Some(whoami::devicename()),
        "cwd" => Some(fmt_cwd()),
        _ => None,
    }
}

fn main() {
    // Create a new Lua state and load the safe subset of the standard libraries.
    let lua = Lua::new();

    // Retreive Lua source code from configuration file.
    lua.load("config = require 'rshx'")
        .exec()
        .expect("Failed to load configuration.");

    // Create configuration table.
    let config: mlua::Table = lua
        .globals()
        .get("config")
        .expect("Failed to create configuration table.");

    // Read, Evaluate, Print and Loop.
    loop {
        // Update prompt string.
        let prompt = resolve_prompt(&config);
        // Display the prompt for the user.
        display_prompt(&prompt);

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
