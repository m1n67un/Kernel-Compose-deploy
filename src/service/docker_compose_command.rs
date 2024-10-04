use std::{
    path::PathBuf, 
    process::Command
};
use clap::Parser;

use crate::{
    interface::docker_compose_command::{
        DockerComposeCommand, 
        DockerComposeCommandTrait
    }, model::{
        args::Args, key_command::KeyCommand, key_db_option::KeyDBOption, key_option::KeyOption
    }
};

impl DockerComposeCommandTrait for DockerComposeCommand {
    fn run(args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(KeyCommand::DOCKER_COMPOSE)
            .args(args)
            .output()?;

        println!("{:?}", output);

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    }

    fn compose_down (current_dir: &PathBuf) -> Result<String, std::io::Error> {
        let args = Args::parse();
        let file_name = args.file.as_str();
        let output = Command::new(KeyCommand::DOCKER_COMPOSE)
            .arg(KeyOption::F)
            .arg(current_dir.join(file_name).to_str().unwrap())
            .arg(KeyCommand::DOWN)
            .arg(KeyDBOption::RMI)
            .arg(KeyCommand::ALL)
            .output()?;

        println!("{:?}", output);

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    }

    fn compose_up (current_dir: &PathBuf) -> Result<String, std::io::Error> {
        let args = Args::parse();
        let file_name = args.file.as_str();
        let output = Command::new(KeyCommand::DOCKER_COMPOSE)
            .arg(KeyOption::F)
            .arg(current_dir.join(file_name).to_str().unwrap())
            .arg(KeyCommand::UP)
            .arg(KeyOption::D)
            .output()?;

        println!("{:?}", output);

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    }
}