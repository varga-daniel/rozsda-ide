use std::process;

pub fn create_new_binary(location: &str, name: &str) -> process::Output {
    process::Command::new("cargo")
        .current_dir(location)
        .args(&["new", "--bin", name])
        .output()
        .expect("Failed to create a new binary using cargo.")
}

pub fn create_new_library(location: &str, name: &str) -> process::Output {
    process::Command::new("cargo")
        .current_dir(location)
        .args(&["new", "--lib", name])
        .output()
        .expect("Failed to create a new library using cargo.")
}

pub fn init_new_binary(location: &str, name: &str) -> process::Output {
    process::Command::new("cargo")
        .current_dir(location)
        .args(&["init", "--bin", name])
        .output()
        .expect("Failed to init a new binary using cargo.")
}

pub fn init_new_library(location: &str, name: &str) -> process::Output {
    process::Command::new("cargo")
        .current_dir(location)
        .args(&["init", "--lib", name])
        .output()
        .expect("Failed to init a new library using cargo.")
}
