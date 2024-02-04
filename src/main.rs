mod app;
mod cli;
mod container;
mod db;
mod install;

use chrono::Utc;
use cli::{app_type, db_type};

use crate::container::Container;
use std::env;

struct Engine {}

impl Engine {
    fn new() -> Engine {
        Engine {}
    }

    pub fn docker_ps(&self) {
        let output = std::process::Command::new("docker")
            .arg("ps")
            .output()
            .expect("failed to execute process");

        // println!("status: {}", output.status);
        println!("{}", String::from_utf8_lossy(&output.stdout));
        // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn create_container(&self, container: Container) {
        container.create();
    }
}

fn compose_name_instance(name: &str) -> String {
    let now = Utc::now();
    format!("{}--{}", name, now.format("%Y%m%d%H%M%S").to_string())
}

pub fn create_db_instance(
    name: String,
    db_type: db::DbType,
    username: String,
    password: String,
    dbname: String,
) -> Container {
    let mut container = Container::new(
        compose_name_instance(&name),
        container::ContainerType::Db { db: db_type },
    );
    container.set_env(username, password, dbname);
    container
}

pub fn create_app_instance(name: String, app_type: app::AppRuntime) -> Container {
    let container = Container::new(
        compose_name_instance(&name),
        container::ContainerType::App { runtime: app_type },
    );
    container
}

fn parse_args(engine: &Engine) -> bool {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return false;
    } else {
        println!(
            "name: {}, type: {}, username: {}, password: {}, dbname: {}",
            args[2], args[3], args[4], args[5], args[6]
        );
        match args[1].as_str() {
            "DB" => {
                let container = create_db_instance(
                    args[2].to_string(),
                    db_type(&args[3]),
                    args[4].to_string(),
                    args[5].to_string(),
                    args[6].to_string(),
                );

                engine.create_container(container);
                return true;
            }
            "App" => {
                let container = create_app_instance(args[2].to_string(), app_type(&args[3]));
                engine.create_container(container);
                return true;
            }
            _ => {
                println!("Invalid choice");
                return false;
            }
        }
    }
}

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
    let engine = Engine::new();

    // Parse the arguments and create the container if needed
    if parse_args(&engine) {
        return;
    }

    // Create a new CLI
    let cli = cli::Cli::new(engine);
    // Run the CLI
    cli.run();
}
