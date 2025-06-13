pub mod core;
pub mod inbuilt;

pub use core::*;
pub use inbuilt::*;

// Helper function for resolving aliases.
pub fn resolve_alias(config: &mlua::Table, cmd: &str) -> Option<String> {
    config
        .get::<mlua::Table>("shellAliases")
        .ok()
        .and_then(|aliases| aliases.get(cmd).ok())
}

// Return prompt from configuration file with expansion performed.
// Otherwise return default prompt.
pub fn resolve_prompt(config: &mlua::Table) -> String {
    let default_prompt = "[${user}@${host}:${dir}]$".to_string();

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
        "dir" => Some(fmt_cwd()),
        _ => None,
    }
}
