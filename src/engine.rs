use chrono::Utc;

use crate::{
    app,
    cli::{app_type, db_type},
    container::{self, Container},
    db,
};
use std::env;

pub struct Engine {}

impl Engine {
    pub fn new() -> Engine {
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

    pub fn parse_args(&self) -> bool {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            return false;
        } else {
            // println!(
            //     "name: {}, type: {}, username: {}, password: {}, dbname: {}",
            //     args[2], args[3], args[4], args[5], args[6]
            // );
            match args[1].as_str() {
                "DB" => {
                    let container = create_db_instance(
                        args[2].to_string(),
                        db_type(&args[3]),
                        args[4].to_string(),
                        args[5].to_string(),
                        args[6].to_string(),
                    );

                    self.create_container(container);
                    return true;
                }
                "App" => {
                    let container = create_app_instance(args[2].to_string(), app_type(&args[3]));
                    self.create_container(container);
                    return true;
                }
                _ => {
                    println!("Invalid choice");
                    return false;
                }
            }
        }
    }

    pub fn create_container(&self, container: Container) {
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
