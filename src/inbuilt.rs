// Provides Built-In Shell Functions.

pub fn cd(args: &[String]) {
    let path = if args.is_empty() {
        std::env::var("HOME").unwrap_or_default()
    } else {
        shellexpand::tilde(&args[0]).to_string()
    };

    let status = std::env::set_current_dir(&path);

    if status.is_err() {
        println!("cd: no such file or directory: {}", &path);
    }
}

pub fn help() {
    println!("rshx v0.1.0");
    println!("---------------------");
    println!("Available Commands:");
    println!("  cd            - Change directory");
    println!("  help          - Display this help message");
    println!("  exit          - Exit the shell");
    println!("  <command>     - Execute a command (e.g., ls, echo, etc.)");

    println!();
    println!("Customization Options:");
    println!("  shellAliases  - Define custom aliases for commands");
    println!("  prompt        - Define custom prompt");

    println!();
    println!("Note: This is a basic implementation. More features and options will be added in future versions.");
}

pub fn exit() {
    std::process::exit(0x0100)
}

