pub struct KeyCommand;

impl KeyCommand {
    pub const DOCKER: &str = "docker";
    pub const DOCKER_COMPOSE: &str = "docker-compose";
    pub const BUILD: &str = "build";
    pub const RUN: &str = "run";
    pub const PS: &str = "ps";
    pub const STOP: &str = "stop";
    pub const IMAGES: &str = "images";
    pub const FILE_NAME: &str = "docker-compose.yml";
    pub const DOWN: &str = "down";
    pub const UP: &str = "up";
    pub const ALL: &str = "all";
}