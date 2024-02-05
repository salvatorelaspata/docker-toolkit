use std::{env::current_dir, fs::create_dir_all, process::Command};

#[derive(Debug, Clone)]
pub enum DbType {
    MySQL,
    PostgreSQL,
    MongoDB,
    Redis,
}

pub struct DB {
    db_type: Option<DbType>,
    name: String,
    username: String,
    password: String,
    dbname: String,
}
impl DB {
    pub fn new(name: String, username: String, password: String, dbname: String) -> DB {
        DB {
            db_type: None,
            name,
            username,
            password,
            dbname,
        }
    }

    pub fn set_type(&mut self, db_type: &DbType) {
        let _db_type: DbType = db_type.clone();
        self.db_type = Some(_db_type);
    }

    pub fn create(&self) -> Result<(), String> {
        let pull = self.pull_image();
        self.create_volume();
        let container = self.run_container();

        if pull.is_ok() && container.is_ok() {
            return Ok(());
        } else {
            return Err(String::from("Failed to create container"));
        }
    }

    fn pull_image(&self) -> Result<(), String> {
        match self.db_type {
            Some(DbType::MySQL) => {
                println!("Pulling MySQL image");

                let output = Command::new("docker")
                    .stdout(std::process::Stdio::inherit())
                    .arg("pull")
                    .arg("mysql")
                    .output()
                    .expect("failed to execute process");

                if output.status.success() {
                    return Ok(());
                } else {
                    return Err(String::from("Failed to pull image"));
                }
            }
            Some(DbType::PostgreSQL) => {
                println!("Pulling PostgreSQL image");
                // exec docker pull postgres
                let output = Command::new("docker")
                    .stdout(std::process::Stdio::inherit())
                    .arg("pull")
                    .arg("postgres")
                    .output()
                    .expect("failed to execute process");

                if output.status.success() {
                    return Ok(());
                } else {
                    return Err(String::from("Failed to pull image"));
                }
            }
            Some(DbType::MongoDB) => {
                println!("Pulling MongoDB image");
                let output = Command::new("docker")
                    .stdout(std::process::Stdio::inherit())
                    .arg("pull")
                    .arg("mongo")
                    .output()
                    .expect("failed to execute process");

                if output.status.success() {
                    return Ok(());
                } else {
                    return Err(String::from("Failed to pull image"));
                }
            }
            Some(DbType::Redis) => {
                println!("Pulling Redis image");
                let output = Command::new("docker")
                    .stdout(std::process::Stdio::inherit())
                    .arg("pull")
                    .arg("redis")
                    .output()
                    .expect("failed to execute process");

                if output.status.success() {
                    return Ok(());
                } else {
                    return Err(String::from("Failed to pull image"));
                }
            }
            None => todo!(),
        }
    }

    fn _get_path(&self) -> String {
        let binding = current_dir().unwrap();
        let current_str = binding.to_str().unwrap();
        let volume_name = format!("{}/volumes/{}_volume", current_str, &self.name);
        volume_name
    }

    fn create_volume(&self) {
        let volume_name = self._get_path();
        let output = create_dir_all(&volume_name);
        match output {
            Ok(_) => {
                println!("Volume created successfully");
            }
            Err(e) => {
                println!("Failed to create volume: {}", e);
            }
        }
    }

    fn run_container(&self) -> Result<(), String> {
        match &self.db_type {
            Some(DbType::MySQL) => {
                println!("Running MySQL container");
                let output = Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.name)
                    .arg("-e")
                    .arg(format!("MYSQL_ROOT_PASSWORD={}", &self.password))
                    .arg("-e")
                    .arg(format!("MYSQL_DATABASE={}", &self.dbname))
                    .arg("-e")
                    .arg(format!("MYSQL_USER={}", &self.username))
                    .arg("-e")
                    .arg(format!("MYSQL_PASSWORD={}", &self.password))
                    .arg("--volume")
                    .arg(format!("{}:/var/lib/mysql", self._get_path()))
                    .arg("-d")
                    .arg("mysql")
                    .output()
                    .expect("failed to execute process");

                if !output.status.success() {
                    return Err(String::from("Failed to run container"));
                } else {
                    return Ok(());
                }
            }
            Some(DbType::PostgreSQL) => {
                println!("Running PostgreSQL container");
                // exec docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
                let output = Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.name)
                    .arg("-e")
                    .arg(format!("POSTGRES_PASSWORD={}", &self.password))
                    .arg("-e")
                    .arg(format!("POSTGRES_USER={}", &self.username))
                    .arg("-e")
                    .arg(format!("POSTGRES_DB={}", &self.dbname))
                    .arg("--volume")
                    .arg(format!("{}:/var/lib/postgresql/data", self._get_path()))
                    .arg("-d")
                    .arg("postgres")
                    .output()
                    .expect("failed to execute process");

                if !output.status.success() {
                    return Err(String::from("Failed to run container"));
                } else {
                    return Ok(());
                }
            }
            Some(DbType::MongoDB) => {
                println!("Running MongoDB container");
                let output = Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.name)
                    .arg("-e")
                    .arg(format!("MONGO_INITDB_ROOT_USERNAME={}", &self.username))
                    .arg("-e")
                    .arg(format!("MONGO_INITDB_ROOT_PASSWORD={}", &self.password))
                    .arg("-e")
                    .arg(format!("MONGO_INITDB_DATABASE={}", &self.dbname))
                    .arg("--volume")
                    .arg(format!("{}:/data/db", self._get_path()))
                    .arg("-d")
                    .arg("mongo")
                    .output()
                    .expect("failed to execute process");

                if !output.status.success() {
                    return Err(String::from("Failed to run container"));
                } else {
                    return Ok(());
                }
            }
            Some(DbType::Redis) => {
                println!("Running Redis container");
                let output = Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.name)
                    .arg("--volume")
                    .arg(format!("{}:/data", self._get_path()))
                    .arg("-d")
                    .arg("redis")
                    .output()
                    .expect("failed to execute process");

                if !output.status.success() {
                    return Err(String::from("Failed to run container"));
                } else {
                    return Ok(());
                }
            }
            None => Err(String::from("DB Type is not set")),
        }
    }

    pub fn get_connection_string(&self, host: String, port: String) -> Result<(), String> {
        let mut connection_string = String::new();
        match &self.db_type {
            Some(DbType::MySQL) => {
                connection_string.push_str("mysql://");
            }
            Some(DbType::PostgreSQL) => {
                connection_string.push_str("postgresql://");
            }
            Some(DbType::MongoDB) => {
                connection_string.push_str("mongodb://");
            }
            Some(DbType::Redis) => {
                connection_string.push_str("redis://");
            }
            None => {
                // Handle the case when db_type is None
            }
        }
        connection_string.push_str(&self.username);
        connection_string.push_str(":");
        connection_string.push_str(&self.password);
        connection_string.push_str("@");
        connection_string.push_str(&host);
        connection_string.push_str(":");
        connection_string.push_str(&port);
        connection_string.push_str("/");
        connection_string.push_str(&self.dbname);

        println!("Connection String: {:?}", connection_string);
        return Ok(());
    }
}
