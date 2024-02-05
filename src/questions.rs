use inquire::{InquireError, Select, Text};

pub struct Question {
    question: String,
    question_type: String,
    answer: String,
}

pub struct Questions {
    questions: Vec<Question>,
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
            match q.question_type.as_str() {
                "text" => {
                    let ans: Result<String, InquireError> = Text::new(&q.question).prompt();
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
                _ => println!("There was an error, please try again"),
            }
        }
        Ok(answers)
    }
}
