use std::env;

pub fn cd(args: &[String]) {
    let path = args[0].to_string();
    assert!(env::set_current_dir(&path).is_ok());
}

pub fn help() {
    println!("RuSH v0.1.0");
    println!("---------------------");
    println!("Available Commands:");
    println!("  cd            - Change directory");
    println!("  help          - Display this help message");
    println!("  exit          - Exit the shell");
    println!("  <command>     - Execute a command (e.g., ls, echo, etc.)");

    println!();
    println!("Customization Options:");
    println!("  shellAliases  - Define custom aliases for commands");

    println!();
    println!("Note: This is a basic implementation. More features and options will be added in future versions.");
}

pub fn exit() {
    std::process::exit(0x0100)
}

