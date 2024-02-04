# rust docker-toolkit

This is an experimental project to create a CLI tool to manage docker containers.



## Usage

There are 2 methods to use this tool:

1. Usage the CLI tool directly --> `cargo run`
2. Usage the CLI without the CLI interface --> `cargo run -- <DB|App> <container_name> <container_type> <container_username> <container_password> <container_dbname>`

```bash
cargo run

# or

cargo run -- <DB|App> <container_name> <container_type> <container_username> <container_password> <container_dbname>

# sample: 

cargo run -- DB myredisdb Redis admin "bcf0f144-cf98-4832-8a82-45c3bac6b067" mydbname
```

> Use the " to wrap the string contains special characters ?!?

## Questions

What do you want to do?

- [X] Create a new container
  - What kind of instance do you want to create?
  - [X] Db
    - which db do you want to create?
      - [X] PostgreSQL
      - [X] MySQL
      - [X] MongoDB
      - [X] Redis
  - [ ] App (wip)
    - which app runtime do you want to create?
      - [ ] Node
      - [ ] Python
      - [ ] Java
      - [ ] Rust
  - [ ] Compose (wip)
    - which app runtime do you want to create?
      - [ ] Node
      - [ ] Python
      - [ ] Java
      - [ ] Rust  
    - witch db do you want to create?
      - [ ] PostgreSQL
      - [ ] MySQL
      - [ ] MongoDB
      - [ ] Redis
- [ ] List all containers
- [ ] Exit

## ToDo

App

Compose