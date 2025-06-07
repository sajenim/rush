use std::env;

pub fn cd(args: Vec<&str>) {
    let path = args[0].to_string();
    assert!(env::set_current_dir(&path).is_ok());
}

pub fn help() {
    println!("This is an example help message")
}

pub fn exit() {
    std::process::exit(0x0100)
}

