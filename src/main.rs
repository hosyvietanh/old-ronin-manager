mod cmd;
mod util;

use std::error::Error;
use std::fs::{self};
use std::path::PathBuf;

fn main() {
    if let Err(e) = set_working_dir() {
        println!(
            "Error happened: {}. Please read instruction in README.md",
            e
        );
        return;
    }
    cmd::commander().run();
}

fn set_working_dir() -> Result<(), Box<dyn Error>> {
    let mut path = std::env::current_exe().unwrap();
    // Get the directory
    path.pop();

    let dir_name = path.file_name().unwrap();

    // In case of cargo run, jump to grand parent dir
    if check_config(&path).is_err() && (dir_name.eq("debug") || dir_name.eq("release")) {
        path.pop();
        path.pop();
    }

    check_config(&path)?;
    std::env::set_current_dir(path)?;
    Ok(())
}

fn check_config(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut has_docker_compose_file = false;
    let mut has_env_file = false;
    let mut has_config_dir = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.path().is_file() && entry.file_name().eq("docker-compose.yml") {
            has_docker_compose_file = true;
        }
        if entry.path().is_file() && entry.file_name().eq(".env") {
            has_env_file = true;
        }
        if entry.path().is_dir() && entry.file_name().eq("config") {
            has_config_dir = true;
        }
    }
    if !has_docker_compose_file {
        Err("docker-compose.yml file is not available".into())
    } else if !has_env_file {
        Err(".env file is not available".into())
    } else if !has_config_dir {
        Err("config directory is not available".into())
    } else {
        Ok(())
    }
}
