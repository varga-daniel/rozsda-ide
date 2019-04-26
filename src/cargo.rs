use std::path::Path;
use std::process;

pub fn init_new_binary(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["init", "--bin"])
                .output()
}

pub fn init_new_library(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["init", "--lib"])
                .output()
}

pub fn build_cargo_project(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["build", "--message-format", "short"])
                .output()
}

pub fn run_cargo_project(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["run", "--message-format", "short"])
                .output()
}

pub fn check_cargo_project(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["check", "--message-format", "short"])
                .output()
}

pub fn clean_cargo_project(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["clean"])
                .output()
}

pub fn test_cargo_project(location: &Path) -> std::io::Result<process::Output> {
        process::Command::new("cargo")
                .current_dir(location)
                .args(&["test", "--message-format", "short"])
                .output()
}
