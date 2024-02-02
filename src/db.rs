struct PostgreSqlEnv {
    postgres_user: String,
    postgres_password: String,
    postgres_db: String,
    // POSTGRES_INITDB_ARGS
    // POSTGRES_INITDB_WALDIR
    // POSTGRES_HOST_AUTH_METHOD
    // PGDATA
}

struct MysqlEnv {
    mysql_user: String,
    mysql_password: String,
    mysql_db: String,
}

struct MongoDbEnv {
    mongo_initdb_root_username: String,
    mongo_initdb_root_password: String,
}

struct RedisEnv {
    redis_password: String,
    redis_dbname: String,
}
#[derive(Debug)]
pub enum DbType {
    MySQL,
    PostgreSQL,
    MongoDB,
    Redis,
}
struct Env {
    MySQL: MysqlEnv,
    PostgreSQL: PostgreSqlEnv,
    MongoDB: MongoDbEnv,
    Redis: RedisEnv,
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

    pub fn set_type(&mut self, db_type: DbType) {
        self.db_type = Some(db_type);
    }

    pub fn create(&self) -> Result<(), String> {
        let pull = self.pull_image();
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
                Err(String::from("Not Implemented"))
            }
            Some(DbType::PostgreSQL) => {
                println!("Pulling PostgreSQL image");
                // exec docker pull postgres
                let output = std::process::Command::new("docker")
                    .arg("pull")
                    .arg("postgres")
                    .output()
                    .expect("failed to execute process");

                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                Ok(())
            }
            Some(DbType::MongoDB) => {
                println!("Pulling MongoDB image");
                Err(String::from("Not Implemented"))
            }
            Some(DbType::Redis) => {
                println!("Pulling Redis image");
                Err(String::from("Not Implemented"))
            }
            None => todo!(),
        }
    }

    pub fn run_container(&self) -> Result<(), String> {
        match &self.db_type {
            Some(DbType::MySQL) => {
                println!("Running MySQL container");
                Err(String::from("Not Implemented"))
            }
            Some(DbType::PostgreSQL) => {
                println!("Running PostgreSQL container");
                // exec docker run --name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
                let output = std::process::Command::new("docker")
                    .arg("run")
                    .arg("--name")
                    .arg(&self.name)
                    .arg("-e")
                    .arg(format!("POSTGRES_PASSWORD={}", &self.password))
                    .arg("-d")
                    .arg("postgres")
                    .output()
                    .expect("failed to execute process");

                println!("status: {}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

                if !output.status.success() {
                    return Err(String::from("Failed to run container"));
                } else {
                    return Ok(());
                }
            }
            Some(DbType::MongoDB) => {
                println!("Running MongoDB container");
                Err(String::from("Not Implemented"))
            }
            Some(DbType::Redis) => {
                println!("Running Redis container");
                Err(String::from("Not Implemented"))
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
