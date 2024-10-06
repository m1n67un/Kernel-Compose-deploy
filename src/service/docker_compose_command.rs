use std::{
    path::PathBuf, 
    process::{
        Command,
        Stdio
    }
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
use std::io::{
    BufRead,
    BufReader
};


impl DockerComposeCommandTrait for DockerComposeCommand {
    fn run(args: &[&str]) -> Result<String, std::io::Error> {
        let output = Command::new(KeyCommand::DOCKER_COMPOSE)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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

    fn compose_down (current_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();
        let file_name = args.file.as_str();
        DockerComposeCommand::run(&[
            KeyOption::F,
            current_dir.join(file_name).to_str().unwrap(),
            KeyCommand::DOWN,
            KeyDBOption::RMI,
            KeyCommand::ALL
        ])?;
        Ok(())
    }

    fn compose_up (current_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();
        let file_name = args.file.as_str();
        let mut child = Command::new("docker-compose")
                            .args(&[
                                KeyOption::F, 
                                current_dir.join(file_name).to_str().unwrap(), 
                                KeyCommand::UP, 
                                KeyOption::D
                            ])
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        for line in stdout_reader.lines() {
            println!("stdout: {}", line?);
        }

        for line in stderr_reader.lines() {
            println!("stderr: {}", line?);
        }

        let status = child.wait()?;
        println!("Exited with status: {}", status);
        Ok(())
    }
}