use std::env;
use crate::interface::{
    docker_command::{
        DockerCommand, 
        DockerCommandTrait
    }, docker_compose_command::{
        DockerComposeCommand, 
        DockerComposeCommandTrait
    }
};

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let _ = DockerCommand::prune();
    let current_dir = env::current_dir()?;
    let _ = DockerComposeCommand::compose_down(&current_dir);
    let _ = DockerCommand::images();
    
    let _ = DockerComposeCommand::compose_up(&current_dir);
    let _ = DockerCommand::list();
    Ok(())
}