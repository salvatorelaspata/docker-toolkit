use inquire::{InquireError, Select, Text};

pub struct Question {
    pub question: String,
    pub question_type: String,
    pub answer: String,
    pub default: String,
}

pub struct Questions {
    pub questions: Vec<Question>,
}

impl Questions {
    pub fn new() -> Self {
        Questions {
            questions: Vec::new(),
        }
    }

    pub fn add(&mut self, question: Question) {
        self.questions.push(question);
    }

    pub fn ask(&self) -> Result<Vec<String>, InquireError> {
        let mut answers: Vec<String> = Vec::new();

        for q in &self.questions {
            // println!("[{}] {}: {}", q.question_type, q.question, q.answer);
            match q.question_type.as_str() {
                "text" => {
                    let ans: Result<String, InquireError>;
                    if q.default == "" {
                        ans = Text::new(&q.question).prompt();
                    } else if q.default == "<uuid>" {
                        ans = Text::new(&q.question)
                            .with_default(&uuid::Uuid::new_v4().to_string())
                            .prompt();
                    } else {
                        ans = Text::new(&q.question).with_default(&q.default).prompt();
                    }

                    match ans {
                        Ok(answer) => answers.push(answer),
                        Err(_) => println!("There was an error, please try again"),
                    }
                }
                "select" => {
                    let options: Vec<&str> = q.answer.split(",").collect();
                    let ans: Result<&str, InquireError> =
                        Select::new(&q.question, options).prompt();
                    match ans {
                        Ok(answer) => answers.push(answer.to_string()),
                        Err(_) => println!("There was an error, please try again"),
                    }
                }
                _ => println!("aaaThere was an error, please try again"),
            }
        }
        Ok(answers)
    }
}
