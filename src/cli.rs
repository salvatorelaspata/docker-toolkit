use crate::{
    app::AppRuntime,
    db::DbType,
    engine::{create_app_instance, create_db_instance, create_sample_compose_instance, Engine},
    i18n::{self, I18n},
    questions,
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
        // self.list_functionalities();
        let mut questions = questions::Questions::new();

        let language = "romano".to_string();
        let i18n = i18n::I18n::new(language);

        let funtionalities_answer = i18n.get("functionalities.answer");
        let funtionalities_answer: Vec<&str> = funtionalities_answer.split(",").collect();

        // i18n.get_all().
        questions.add(questions::Question {
            question: i18n.get("functionalities.question"),
            question_type: i18n.get("functionalities.question_type"),
            answer: i18n.get("functionalities.answer"),
            default: String::from(""),
        });
        let answers = questions.ask().unwrap();
        if answers[0] == funtionalities_answer[0] {
            let mut questions_container = questions::Questions::new();
            // let rng = RNG::try_from(&Language::Roman).unwrap();
            questions_container.add(questions::Question {
                question: i18n.get("container_name.question"),
                question_type: i18n.get("container_name.question_type"),
                answer: i18n.get("container_name.answer"),
                default: String::from(""), //rng.generate_name()
            });
            questions_container.add(questions::Question {
                question: i18n.get("container_type.question"),
                question_type: i18n.get("container_type.question_type"),
                answer: i18n.get("container_type.answer"),
                default: String::from(""),
            });
            let answers_container = questions_container.ask().unwrap();

            let db_app = i18n.get("container_type.answer");
            let db_app: Vec<&str> = db_app.split(",").collect();
            let db_app_answer = answers_container[1].to_string();
            let _db = db_app[0];
            let _app = db_app[1];
            let _compose = db_app[2];
            if db_app_answer == _db {
                let mut questions_db = questions::Questions::new();
                questions_db.add(questions::Question {
                    question: i18n.get("db_container_type.question"),
                    question_type: i18n.get("db_container_type.question_type"),
                    answer: i18n.get("db_container_type.answer"),
                    default: String::from(""),
                });

                questions_db.add(questions::Question {
                    question: i18n.get("db_container_username.question"),
                    question_type: i18n.get("db_container_username.question_type"),
                    answer: i18n.get("db_container_username.answer"),
                    default: String::from("admin"),
                });
                questions_db.add(questions::Question {
                    question: i18n.get("db_container_password.question"),
                    question_type: i18n.get("db_container_password.question_type"),
                    answer: i18n.get("db_container_password.answer"),
                    default: String::from("<uuid>"),
                });
                questions_db.add(questions::Question {
                    question: i18n.get("db_container_dbname.question"),
                    question_type: i18n.get("db_container_dbname.question_type"),
                    answer: i18n.get("db_container_dbname.answer"),
                    default: String::from("mydb"),
                });

                let answers_db = questions_db.ask().unwrap();
                let container = create_db_instance(
                    answers_container[0].to_string(),
                    db_type(&answers_db[0]),
                    answers_db[1].to_string(),
                    answers_db[2].to_string(),
                    answers_db[3].to_string(),
                );
                self.engine.create_container(container);
                self.you_want_to_continue(i18n)
            } else if db_app_answer == _app {
                let mut questions_app = questions::Questions::new();

                questions_app.add(questions::Question {
                    question: i18n.get("create_app_container.question"),
                    question_type: i18n.get("create_app_container.question_type"),
                    answer: i18n.get("create_app_container.answer"),
                    default: String::from(""),
                });

                let answers_app = questions_app.ask().unwrap();
                let container = create_app_instance(
                    answers_container[0].to_string(),
                    app_type(&answers_app[0]),
                );
                // block stdin during the container creation
                self.engine.create_container(container);
                self.you_want_to_continue(i18n)
            } else if db_app_answer == _compose {
                // TODO - questions for compose to be added here
                let compose = create_sample_compose_instance(answers_container[0].to_string());

                self.engine.create_compose_instance(compose);
                self.you_want_to_continue(i18n)
            }
        } else if answers[0] == funtionalities_answer[1] {
            self.list_containers(i18n);
        } else {
            println!("Goodbye!");
        }
    }
    // 2.2 List all containers
    fn list_containers(&self, i18n: I18n) {
        println!("Listing all containers");
        self.engine.docker_ps();
        self.you_want_to_continue(i18n)
    }
    fn you_want_to_continue(&self, i18n: I18n) {
        let yes_no = i18n.get("yes_no_continue.answer");
        let yes_no: Vec<&str> = yes_no.split(",").collect();
        let mut _questions = questions::Questions::new();
        _questions.add(questions::Question {
            question: i18n.get("yes_no_continue.question"),
            question_type: i18n.get("yes_no_continue.question_type"),
            answer: i18n.get("yes_no_continue.answer"),
            default: String::from(""),
        });
        let answers = _questions.ask().unwrap();
        if answers[0] == yes_no[0] {
            self.run();
        } else {
            println!("Goodbye!");
        }
    }
}
