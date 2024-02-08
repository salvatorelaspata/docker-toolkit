struct Instruction {
    name: String,
    args: Vec<String>,
}
pub struct Dockerfile {
    instructions: Vec<Instruction>,
}

impl Dockerfile {
    pub fn new() -> Dockerfile {
        Dockerfile {
            instructions: Vec::new(),
        }
    }

    fn add_instruction(&mut self, name: String, args: Vec<String>) {
        let instruction = Instruction { name, args };
        self.instructions.push(instruction);
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for instruction in &self.instructions {
            result.push_str(&instruction.name);
            result.push_str(" ");
            for arg in &instruction.args {
                result.push_str(&arg);
                result.push_str(" ");
            }
            result.push_str("\n");
        }
        result
    }
}

pub fn create_node_dockerfile() -> Dockerfile {
    let mut dockerfile = Dockerfile::new();
    // node
    dockerfile.add_instruction(
        String::from("FROM"),
        vec![
            "node:lts".to_string(),
            "AS".to_string(),
            "build".to_string(),
        ],
    );
    dockerfile.add_instruction(String::from("WORKDIR"), vec!["/app".to_string()]);
    dockerfile.add_instruction(
        String::from("COPY"),
        vec!["package*.json".to_string(), "./".to_string()],
    );
    dockerfile.add_instruction(
        String::from("RUN"),
        vec!["npm".to_string(), "install".to_string()],
    );
    dockerfile.add_instruction(String::from("COPY"), vec![".".to_string(), ".".to_string()]);
    dockerfile.add_instruction(
        String::from("RUN"),
        vec!["npm".to_string(), "run".to_string(), "build".to_string()],
    );
    // nginx
    dockerfile.add_instruction(
        String::from("FROM"),
        vec![
            "nginx:alpine".to_string(),
            "AS".to_string(),
            "runtime".to_string(),
        ],
    );
    dockerfile.add_instruction(
        String::from("COPY"),
        vec![
            "./nginx/nginx.conf".to_string(),
            "/etc/nginx/nginx.conf".to_string(),
        ],
    );
    dockerfile.add_instruction(
        String::from("COPY"),
        vec![
            "--from=build".to_string(),
            "/app/dist".to_string(),
            "/usr/share/nginx/html".to_string(),
        ],
    );
    dockerfile.add_instruction(String::from("EXPOSE"), vec!["4321".to_string()]);
    dockerfile
}

pub fn create_simple_dockerfile() -> Dockerfile {
    // return a simple dockerfile with nodejs image and run build
    let mut dockerfile = Dockerfile::new();
    dockerfile.add_instruction(String::from("FROM"), vec!["node:lts".to_string()]);
    dockerfile.add_instruction(String::from("WORKDIR"), vec!["/app".to_string()]);
    dockerfile.add_instruction(
        String::from("COPY"),
        vec![
            "package.json".to_string(),
            "package-lock.json".to_string(),
            "./".to_string(),
        ],
    );
    dockerfile.add_instruction(String::from("RUN"), vec!["npm install".to_string()]);
    dockerfile.add_instruction(String::from("COPY"), vec![".".to_string(), ".".to_string()]);
    dockerfile.add_instruction(String::from("CMD"), vec!["npm start".to_string()]);

    println!("{}", dockerfile.to_string());
    dockerfile
}
