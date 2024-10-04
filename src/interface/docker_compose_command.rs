use std::path::PathBuf;

pub struct DockerComposeCommand;
pub trait DockerComposeCommandTrait {
    fn run(args: &[&str]) -> Result<String, std::io::Error>;
    fn compose_down (current_dir: &PathBuf) -> Result<String, std::io::Error>;
    fn compose_up (current_dir: &PathBuf) -> Result<String, std::io::Error>;
}