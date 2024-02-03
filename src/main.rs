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

        // let is_db = match container.container_type {
        //     ContainerType::Db { db } => true,
        //     _ => false,
        // };

        // if is_db {
        //     // set user, password, db
        //     // container.set_db_config(
        //     //     String::from(" "),
        //     //     String::from("container.password"),
        //     //     String::from("container.db_name"),
        //     // );
        // } else {
        //     // create app
        // }
        // container.set_db_config(
        //     String::from(" "),
        //     String::from("container.password"),
        //     String::from("container.db_name"),
        // );
        container.create();

        self.containers.push(container);
    }
}

// macro to create a new container
macro_rules! create_container {
    ($name:expr, $description:expr, $version:expr, $container_type:expr) => {
        Container::new(
            $name.to_string(),
            $description.to_string(),
            $version.to_string(),
            $container_type,
        )
    };
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

    let mysql = create_container!(
        format!("{}--{}", String::from("mysql"), date_as_string),
        // "admin",
        // "admin",
        // "mydb",
        "MySQL Database",
        "latest",
        ContainerType::Db { db: DbType::MySQL }
    );

    let postgres = create_container!(
        format!("{}--{}", String::from("postgres"), date_as_string),
        // "admin",
        // "admin",
        // "mydb",
        "Postgres Database",
        "latest",
        ContainerType::Db {
            db: DbType::PostgreSQL
        }
    );

    let mongodb = create_container!(
        format!("{}--{}", String::from("mongodb"), date_as_string),
        // "admin",
        // "admin",
        // "mydb",
        "MongoDB Database",
        "latest",
        ContainerType::Db {
            db: DbType::MongoDB
        }
    );

    let redis = create_container!(
        format!("{}--{}", String::from("redis"), date_as_string),
        // "admin",
        // "admin",
        // "mydb",
        "Redis Database",
        "latest",
        ContainerType::Db { db: DbType::Redis }
    );

    // nodejs.create();

    let mut engine = Engine::new();
    // db
    engine.create_container(postgres);
    engine.create_container(mongodb);
    engine.create_container(redis);
    engine.create_container(mysql);

    engine.ps();

    engine.docker_ps();
}
