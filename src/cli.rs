use chrono::prelude::*;
use inquire::{InquireError, Select, Text};

use crate::{app::AppRuntime, container, db::DbType, Engine};

pub struct Cli {
    engine: Engine,
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

                fn db_type(string: &str) -> DbType {
                    match string {
                        "PostgreSQL" => DbType::PostgreSQL,
                        "MySQL" => DbType::MySQL,
                        "MongoDB" => DbType::MongoDB,
                        "Redis" => DbType::Redis,
                        _ => DbType::PostgreSQL,
                    }
                }

                match ans {
                    Ok(name) => {
                        // create name from text + ts
                        let now = Utc::now();
                        let date_as_string = now.format("%Y%m%d%H%M%S").to_string();
                        let container = container::Container::new(
                            format!("{}--{}", name, date_as_string),
                            "description".to_string(),
                            "version".to_string(),
                            container::ContainerType::Db {
                                db: db_type(&choice),
                            },
                        );
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

                fn db_type(string: &str) -> AppRuntime {
                    match string {
                        "Node" => AppRuntime::Node,
                        "Python" => AppRuntime::Python,
                        "Java" => AppRuntime::Java,
                        "Rust" => AppRuntime::Rust,
                        _ => AppRuntime::Node,
                    }
                }

                match ans {
                    Ok(name) => {
                        let now = Utc::now();
                        let date_as_string = now.format("%Y%m%d%H%M%S").to_string();
                        let container = container::Container::new(
                            format!("{}--{}", name, date_as_string),
                            "description".to_string(),
                            "version".to_string(),
                            container::ContainerType::App {
                                runtime: db_type(&choice),
                            },
                        );
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
