mod app;
mod cli;
mod container;
mod db;
mod engine;
mod install;

use crate::engine::Engine;
fn main() {
    println!("Docker toolkit!");

    // INSTALLED
    let is_installed = install::check_if_docker_is_installed();
    if is_installed.is_err() {
        return println!("Docker error: {:?}", is_installed.err().unwrap());
    }
    println!("Docker is installed");

    // RUNNING
    let runnig = install::check_if_docker_is_running();
    if runnig.is_err() {
        return println!("Docker error: {:?}", runnig.err().unwrap());
    }
    println!("Docker still running");
    // Create a new engine
    let _engine = Engine::new();

    // Parse the arguments and create the container if needed
    if _engine.parse_args() {
        return;
    }

    // Create a new CLI
    let cli = cli::Cli::new(_engine);

    // Run the CLI
    cli.run();
}
