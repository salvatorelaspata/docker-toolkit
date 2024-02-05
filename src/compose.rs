// create a new docker-compose.yml file

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn _write_line(file: &mut File, line: &str) {
    match file.write_all(line.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to file: {}", why)
        }
        Ok(_) => println!("successfully wrote to file"),
    }
}

fn create_compose_file() {
    let path = Path::new("docker-compose.yml");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    _write_line(&mut file, "version: '3'\n");
    _write_line(&mut file, "services:\n");
}

fn create_service(service_name: &str, image: &str, ports: &str, volumes: &str) {
    let path = Path::new("docker-compose.yml");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    _write_line(&mut file, &format!("  {}:\n", service_name));
    _write_line(&mut file, &format!("    image: {}\n", image));
    _write_line(&mut file, "    ports:\n");
    _write_line(&mut file, &format!("      - \"{}:8080\"\n", ports));
    _write_line(&mut file, "    volumes:\n");
    _write_line(&mut file, &format!("      - {}:/app\n", volumes));
    // _write_line(&mut file, "    environment:\n");
    // _write_line(&mut file, "      - DEBUG=1\n");
    // _write_line(&mut file, "    depends_on:\n");
    // _write_line(&mut file, "      - db\n");
    _write_line(&mut file, "\n");
}

fn main() {
    create_compose_file();
    create_service("web");
    create_service("db");
}
