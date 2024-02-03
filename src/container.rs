use crate::{
    app::AppRuntime,
    db::{DbType, DB},
};

pub enum ContainerType {
    Db { db: DbType },
    App { runtime: AppRuntime },
}

pub struct Container {
    pub name: String,
    pub description: String,
    pub version: String,
    pub container_type: ContainerType,
}
impl Container {
    pub fn new(
        name: String,
        description: String,
        version: String,
        container_type: ContainerType,
    ) -> Container {
        Container {
            name,
            description,
            version,
            container_type,
        }
    }

    pub fn create(&self) {
        let host = String::from("localhost");
        let port = String::from("5432");
        match &self.container_type {
            ContainerType::Db { db } => {
                // create not referenced db
                let mut _db = DB::new(
                    self.name.clone(),
                    "root".to_string(),
                    "password".to_string(),
                    "mydb".to_string(),
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
