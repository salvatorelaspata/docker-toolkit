mod app;
mod cli;
mod container;
mod db;
mod install;

use crate::container::Container;

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

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    fn create_container(&self, container: Container) {
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

    let engine = Engine::new();

    let cli = cli::Cli::new(engine);
    cli.run();
}
