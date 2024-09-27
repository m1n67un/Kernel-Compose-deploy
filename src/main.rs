use handler::docker_build;

mod model;
mod interface;
mod service;
mod handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    docker_build::build()
}