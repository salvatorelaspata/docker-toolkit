use chrono::prelude::*;

mod app;
mod container;
mod db;
mod install;

use crate::{
    app::AppRuntime,
    container::{Container, ContainerType},
    db::DbType,
};

struct Engine {
    containers: Vec<Container>,
}

impl Engine {
    fn new() -> Engine {
        Engine { containers: vec![] }
    }

    fn ps(&self) {
        for container in &self.containers {
            println!("{:?}", container.name);
        }
    }

    fn docker_ps(&self) {
        let output = std::process::Command::new("docker")
            .arg("ps")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn create_container(&mut self, container: Container) {
        let container = container::Container::new(
            container.name,
            container.description,
            container.version,
            container.container_type,
        );
        container.create();
        self.containers.push(container);
    }
}

fn main() {
    println!("Hello, world!");
    let is_installed = install::check_if_docker_is_installed();
    println!("Docker is installed: {:?}", is_installed.to_string());
    if !is_installed {
        return println!("Install docker!"); // install::install_docker();
    }
    let runnig = install::check_if_docker_is_running();
    println!("Docker is running: {:?}", runnig);

    // create name from text + ts
    let now = Utc::now();
    let date_as_string = now.format("%Y%m%d%H%M%S").to_string();

    let db_name = format!("{}--{}", String::from("postgres"), date_as_string);
    let app_name = format!("{}--{}", String::from("nodejs"), date_as_string);
    println!("DB Name: {}", db_name);
    println!("App Name: {}", app_name);
    let postgres = Container::new(
        db_name,
        "Postgres Database".to_string(),
        "latest".to_string(),
        ContainerType::Db {
            db: DbType::PostgreSQL,
        },
    );

    let nodejs = Container::new(
        app_name,
        "".to_string(),
        "latest".to_string(),
        ContainerType::App {
            runtime: AppRuntime::Node,
        },
    );

    // nodejs.create();

    let mut engine = Engine::new();
    engine.create_container(postgres);

    engine.create_container(nodejs);

    engine.ps();

    engine.docker_ps();
}
