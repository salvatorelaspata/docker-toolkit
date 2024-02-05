use inquire::{InquireError, Select, Text};
use uuid::Uuid;

use crate::{
    app::AppRuntime, db::DbType, engine::create_app_instance, engine::create_db_instance, Engine,
};

pub struct Cli {
    engine: Engine,
}

pub fn db_type(string: &str) -> DbType {
    match string {
        "PostgreSQL" => DbType::PostgreSQL,
        "MySQL" => DbType::MySQL,
        "MongoDB" => DbType::MongoDB,
        "Redis" => DbType::Redis,
        _ => DbType::PostgreSQL,
    }
}

pub fn app_type(string: &str) -> AppRuntime {
    match string {
        "Node" => AppRuntime::Node,
        "Python" => AppRuntime::Python,
        "Java" => AppRuntime::Java,
        "Rust" => AppRuntime::Rust,
        _ => AppRuntime::Node,
    }
}

impl Cli {
    pub fn new(engine: Engine) -> Cli {
        Cli { engine }
    }

    pub fn run(&self) {
        self.list_functionalities();
    }
    // 1. List of functionalities
    fn list_functionalities(&self) {
        let choise: Vec<&str> = vec!["Create a new container", "List all containers", "Exit"];

        let ans: Result<&str, InquireError> =
            Select::new("What do you want to do?", choise).prompt();

        match ans {
            Ok(choice) => match choice {
                "Create a new container" => self.create_container(),
                "List all containers" => self.list_containers(),
                "Exit" => println!("Goodbye!"),
                _ => println!("Invalid choice"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
    // 2.1 Create a new container
    fn create_container(&self) {
        println!("Creating a new container");
        let options: Vec<&str> = vec!["DB", "App"];

        let ans: Result<&str, InquireError> =
            Select::new("What kind of instance do you want to create?", options).prompt();

        match ans {
            Ok(choice) => match choice {
                "DB" => self.create_db_container(),
                "App" => self.create_app_container(),
                _ => println!("Invalid choice"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
    // 2.2 List all containers
    fn list_containers(&self) {
        println!("Listing all containers");
        self.engine.docker_ps();
        self.you_want_to_continue()
    }
    // 3.1 Create a new db container
    fn create_db_container(&self) {
        let options: Vec<&str> = vec!["PostgreSQL", "MySQL", "MongoDB", "Redis"];

        let ans: Result<&str, InquireError> =
            Select::new("which db do you want to create?", options).prompt();

        match ans {
            Ok(choice) => {
                println!("{}! That's mine too!", choice);
                let ans: Result<String, InquireError> =
                    Text::new("What's the instance name?").prompt();

                match ans {
                    Ok(name) => {
                        let username: String = Text::new("What's the username?")
                            .with_default("admin")
                            .prompt()
                            .unwrap();
                        let password: String = Text::new("What's the password?")
                            .with_default(&Uuid::new_v4().to_string())
                            .prompt()
                            .unwrap();
                        let dbname: String = Text::new("What's the dbname?")
                            .with_default("mydb")
                            .prompt()
                            .unwrap();

                        let container =
                            create_db_instance(name, db_type(&choice), username, password, dbname);
                        self.engine.create_container(container);
                        self.you_want_to_continue()
                    }
                    Err(_) => println!("There was an error, please try again"),
                }
            } // CREATE CONTAINER
            Err(_) => println!("There was an error, please try again"),
        }
    }
    // 3.2 Create an app container
    fn create_app_container(&self) {
        let options: Vec<&str> = vec!["Node", "Python", "Java", "Rust"];

        let ans: Result<&str, InquireError> =
            Select::new("which app runtimee do you want to create?", options).prompt();

        match ans {
            Ok(choice) => println!("{}! That's mine too!", choice),
            Err(_) => println!("There was an error, please try again"),
        }

        let ans: Result<String, InquireError> = Text::new("What's the instance name?").prompt();

        match ans {
            Ok(choice) => {
                println!("{}! That's mine too!", choice);
                let ans: Result<String, InquireError> =
                    Text::new("What's the instance name?").prompt();

                match ans {
                    Ok(name) => {
                        let container = create_app_instance(name, app_type(&choice));
                        self.engine.create_container(container);
                        self.you_want_to_continue()
                    }
                    Err(_) => println!("There was an error, please try again"),
                }
            } // CREATE CONTAINER
            Err(_) => println!("There was an error, please try again"),
        }
    }

    fn you_want_to_continue(&self) {
        let choise: Vec<&str> = vec!["Yes", "No"];

        let ans: Result<&str, InquireError> =
            Select::new("Do you want to continue?", choise).prompt();

        match ans {
            Ok(choice) => match choice {
                "Yes" => self.run(),
                "No" => println!("Goodbye!"),
                _ => println!("Invalid choice"),
            },
            Err(_) => println!("There was an error, please try again"),
        }
    }
}
