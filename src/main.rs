use app::AppRuntime;
use db::DbType;

use crate::db::DB;

mod app;
mod db;
mod install;

enum DockerType {
    Db { db: DbType },
    App { runtime: AppRuntime },
}

struct Docker {
    name: String,
    description: String,
    version: String,
    docker_type: DockerType,
}
impl Docker {
    fn new(name: String, description: String, version: String, docker_type: DockerType) -> Docker {
        Docker {
            name,
            description,
            version,
            docker_type,
        }
    }

    fn create(&self) {
        let host = String::from("localhost");
        let port = String::from("5432");
        match &self.docker_type {
            DockerType::Db { db } => {
                // create not referenced db

                let mut _db = DB::new(
                    self.name.clone(),
                    "root".to_string(),
                    "password".to_string(),
                    "mydb".to_string(),
                );

                _db.set_type(DbType::PostgreSQL);
                println!("Creating a new database: {:?}", _db.create());

                println!(
                    "Connection String: {:?}",
                    _db.get_connection_string(host, port)
                );
            }
            DockerType::App { runtime } => {
                println!("Creating a new app runtime: {:?}", runtime);
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    let is_installed = install::check_if_docker_is_installed();
    println!("Docker is installed: {:?}", is_installed.to_string());
    if !is_installed {
        install::install_docker();
    }
    let runnig = install::check_if_docker_is_running();
    println!("Docker is running: {:?}", runnig);

    let postgres = Docker::new(
        "postgres".to_string(),
        "Postgres Database".to_string(),
        "latest".to_string(),
        DockerType::Db {
            db: DbType::PostgreSQL,
        },
    );

    postgres.create();

    let nodejs = Docker::new(
        "nodejs".to_string(),
        "".to_string(),
        "latest".to_string(),
        DockerType::App {
            runtime: AppRuntime::Node,
        },
    );

    nodejs.create();
}

// fn to return current OS
