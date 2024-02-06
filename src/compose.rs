use std::env::current_dir;

pub struct Service {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub networks: Vec<String>,
    pub volumes: Vec<String>,
}

pub struct Network {
    pub name: String,
}

pub struct Volume {
    pub name: String,
}

pub struct Compose {
    pub name: String,
    pub services: Vec<Service>,
    pub networks: Vec<Network>,
    pub volumes: Vec<Volume>,
}

impl Compose {
    pub fn new(name: String) -> Compose {
        Compose {
            name,
            services: Vec::new(),
            networks: Vec::new(),
            volumes: Vec::new(),
        }
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.push(service);
    }

    pub fn add_network(&mut self, network: Network) {
        self.networks.push(network);
    }

    pub fn add_volume(&mut self, volume: Volume) {
        self.volumes.push(volume);
    }
    fn _get_path(&self) -> String {
        let binding = current_dir().unwrap();
        let current_str = binding.to_str().unwrap();
        let volume_name = format!("{}/dockercompose/{}.yml", current_str, &self.name);
        volume_name
    }

    pub fn create(&self) {
        let path = self._get_path();
        let content = self.to_string();
        std::fs::write(path, content).unwrap();
    }

    pub fn to_string(&self) -> String {
        println!("Compose to_string");
        let mut result = String::new();
        result.push_str("version: '3'\n");

        result.push_str("services:\n");
        for service in &self.services {
            result.push_str(&service.to_string());
        }

        result.push_str("networks:\n");
        for network in &self.networks {
            result.push_str(&network.to_string());
        }
        result.push_str("volumes:\n");
        for volume in &self.volumes {
            result.push_str(&volume.to_string());
        }
        result
    }
}

impl Service {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("  {}:\n", self.name));
        result.push_str(&format!("    image: {}\n", self.image));
        if !self.ports.is_empty() {
            result.push_str("    ports:\n");
            for port in &self.ports {
                result.push_str(&format!("      - {}\n", port));
            }
        }
        if !self.networks.is_empty() {
            result.push_str("    networks:\n");
            for network in &self.networks {
                result.push_str(&format!("      - {}\n", network));
            }
        }
        if !self.volumes.is_empty() {
            result.push_str("    volumes:\n");
            for volume in &self.volumes {
                result.push_str(&format!("      - {}\n", volume));
            }
        }
        result
    }
}

impl Network {
    pub fn to_string(&self) -> String {
        format!("  {}:\n", self.name)
    }
}

impl Volume {
    pub fn to_string(&self) -> String {
        format!("  {}:\n", self.name)
    }
}
