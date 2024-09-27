use std::{path::PathBuf, process::Command};

use crate::{
    interface::docker_command::{
        DockerCommand, 
        DockerCommandTrait
    }, 
    model::{
        key_command::KeyCommand, 
        key_db_option::KeyDBOption, 
        key_option::KeyOption
    }
};

impl DockerCommandTrait for DockerCommand {
    fn run(args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(KeyCommand::DOCKER)
            .args(args)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    }

    fn image_build(current_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        DockerCommand::run(&[KeyCommand::BUILD, KeyOption::T, "rust-docker-image", current_dir.to_str().unwrap()])?;
        Ok(())
    }

    fn container() -> Result<(), Box<dyn std::error::Error>> {
        DockerCommand::run(&[KeyCommand::RUN, KeyDBOption::RM, KeyOption::D, KeyDBOption::NAME, "rust-docker-container", "rust-docker-image"])?;
        Ok(())
    }

    fn list() -> Result<(), Box<dyn std::error::Error>> {
        DockerCommand::run(&[KeyCommand::PS])?;
        Ok(())
    }

    fn stop() -> Result<(), Box<dyn std::error::Error>> {
        DockerCommand::run(&[KeyCommand::STOP, "rust-docker-container"])?;
        Ok(())
    }

    fn images() -> Result<(), Box<dyn std::error::Error>> {
        DockerCommand::run(&[KeyCommand::IMAGES])?;
        Ok(())
    }
}