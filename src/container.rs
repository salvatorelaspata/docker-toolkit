use crate::{
    app::AppRuntime,
    db::{DbType, DB},
};

pub enum ContainerType {
    Db { db: DbType },
    App { runtime: AppRuntime },
}

struct Config {
    username: String,
    password: String,
    dbname: String,
}

pub struct Container {
    pub name: String,
    pub container_type: ContainerType,
    // optional configuration environments
    env: Config,
}
impl Container {
    pub fn new(name: String, container_type: ContainerType) -> Container {
        Container {
            name,
            container_type,
            env: Config {
                username: String::from("admin"),
                password: String::from("64f17b9e-2808-43cd-a9f0-c1569fb40823"),
                dbname: String::from("mydb"),
            },
            // default env
        }
    }

    pub fn set_env(&mut self, username: String, password: String, dbname: String) {
        self.env = Config {
            username,
            password,
            dbname,
        };
    }

    pub fn create(&self) {
        let host = String::from("localhost");
        let port = String::from("5432");

        match &self.container_type {
            ContainerType::Db { db } => {
                // create not referenced db
                let mut _db = DB::new(
                    self.name.clone(),
                    self.env.username.clone(),
                    self.env.password.clone(),
                    self.env.dbname.clone(),
                );

                _db.set_type(db);

                let result = _db.create();
                if result.is_ok() {
                    println!("Container {} created successfully", self.name);
                } else {
                    println!("Failed to create container {}", self.name);
                }

                println!(
                    "Connection String: {:?}",
                    _db.get_connection_string(host, port)
                );
            }
            ContainerType::App { runtime } => {
                println!("Creating a new app runtime: {:?}", runtime);
            }
        }
    }
}
