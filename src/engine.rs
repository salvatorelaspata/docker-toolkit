use crate::{
    app,
    cli::{app_type, db_type},
    compose::{Compose, Network, Service, ServiceType, Volume},
    container::{self, Container},
    db, i18n,
};
use chrono::Utc;
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
            let db_app = i18n::I18n::new("romano".to_string()).get("container_type.answer");
            let db_app: Vec<&str> = db_app.split(",").collect();

            let _db = db_app[0];
            let _app = db_app[1];
            let _compose = db_app[2];
            if args[1].as_str() == _db {
                let container = create_db_instance(
                    args[2].to_string(),
                    db_type(&args[3]),
                    args[4].to_string(),
                    args[5].to_string(),
                    args[6].to_string(),
                );

                self.create_container(container);
                return true;
            } else if args[1].as_str() == _app {
                let container = create_app_instance(args[2].to_string(), app_type(&args[3]));
                self.create_container(container);
                return true;
            } else if args[1].as_str() == _compose {
                let compose = create_sample_compose_instance("sample".to_string());
                self.create_compose_instance(compose);
                return true;
            }

            println!("Invalid choice");
            return false;
        }
    }

    pub fn create_container(&self, container: Container) {
        container.create();
    }

    pub fn create_compose_instance(&self, compose: Compose) {
        compose.create();
    }
}

pub fn compose_name_instance(name: &str) -> String {
    let now = Utc::now();
    format!("{}--{}", name, now.format("%Y%m%d%H%M%S").to_string())
}

pub fn create_sample_compose_instance(name: String) -> Compose {
    // create a simple docker-compose file with a web service and a db service
    let mut compose = Compose::new(compose_name_instance(&name));

    let nginx_service = Service {
        service_type: ServiceType::APP,
        name: "web".to_string(),
        image: "nginx".to_string(),
        ports: vec!["80:80".to_string()],
        networks: vec!["frontend".to_string(), "backend".to_string()],
        volumes: vec!["/var/www".to_string()],
    };
    compose.add_service(nginx_service);

    let db_service = Service {
        service_type: ServiceType::DB,
        name: "db".to_string(),
        image: "postgres".to_string(),
        ports: vec!["5432:5432".to_string()],
        networks: vec!["backend".to_string()],
        volumes: vec!["/var/lib/postgresql/data".to_string()],
    };

    compose.add_service(db_service);

    let frontend_network = Network {
        name: "frontend".to_string(),
    };

    compose.add_network(frontend_network);

    let backend_network = Network {
        name: "backend".to_string(),
    };

    compose.add_network(backend_network);

    let data_volume = Volume {
        name: "data".to_string(),
    };

    compose.add_volume(data_volume);

    compose
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
