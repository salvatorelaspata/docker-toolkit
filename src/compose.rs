// // manage docker-compose commands

// struct DockerCompose {
//     name: String,
//     description: String,
//     version: String,
//     services: Vec<Docker>,
// }

// impl DockerCompose {
//     fn new(
//         name: String,
//         description: String,
//         version: String,
//         services: Vec<String>,
//     ) -> DockerCompose {
//         DockerCompose {
//             name,
//             description,
//             version,
//             services,
//         }
//     }

//     fn create(&self) {
//         println!("Creating a new docker-compose: {:?}", self);
//     }
// }

// fn main() {
//     let db = Docker::new(
//         String::from("PostgreSQL"),
//         String::from("PostgreSQL Database"),
//         String::from("12.0"),
//         DockerType::Db {
//             db: DbType::PostgreSQL,
//         },
//     );

//     let app = Docker::new(
//         String::from("AppRuntime"),
//         String::from("App Runtime"),
//         String::from("1.0"),
//         DockerType::App {
//             runtime: AppRuntime::new(
//                 String::from("AppRuntime"),
//                 String::from("App Runtime"),
//                 String::from("1.0"),
//             ),
//         },
//     );

//     let compose = DockerCompose::new(
//         String::from("DockerCompose"),
//         String::from("Docker Compose"),
//         String::from("1.0"),
//         vec![String::from("PostgreSQL"), String::from("AppRuntime")],
//     );

//     db.create();
//     app.create();
//     compose.create();
// }
