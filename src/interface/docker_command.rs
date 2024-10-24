use std::path::PathBuf;

pub struct DockerCommand;
pub trait DockerCommandTrait {
    fn run(args: &[&str]) -> Result<String, std::io::Error>;
    fn image_build(current_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;
    fn container() -> Result<(), Box<dyn std::error::Error>>;
    fn list() -> Result<(), Box<dyn std::error::Error>>;
    fn stop() -> Result<(), Box<dyn std::error::Error>>;
    fn images() -> Result<(), Box<dyn std::error::Error>>;
    fn prune() -> Result<(), Box<dyn std::error::Error>>;
}