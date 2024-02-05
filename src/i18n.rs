use std::{collections::HashMap, env::current_dir, fs::File, io::BufReader};

use serde_json::Value;

pub struct I18n {
    pub language: String,
    pub messages: HashMap<String, String>,
}

impl I18n {
    pub fn new(language: String) -> Self {
        let mut messages = HashMap::new();
        let binding = current_dir().unwrap();
        let current_str = binding.to_str().unwrap();
        let path = format!("{}/i18n/{}.json", current_str, language);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let data: Value = serde_json::from_reader(reader).unwrap();

        fn recursive_insert(
            messages: &mut HashMap<String, String>,
            data: &Value,
            key: String,
        ) -> () {
            match data {
                Value::Object(map) => {
                    for (k, v) in map {
                        let new_key = if key.is_empty() {
                            k.to_string()
                        } else {
                            format!("{}.{}", key, k)
                        };
                        recursive_insert(messages, v, new_key);
                    }
                }
                _ => {
                    messages.insert(key, data.to_string());
                }
            }
        }

        recursive_insert(&mut messages, &data, "".to_string());
        I18n { language, messages }
    }

    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.messages
    }

    pub fn get(&self, key: &str) -> String {
        match self.messages.get(key) {
            Some(value) => value.trim_matches('"').to_owned(),
            None => "Key not found".to_string(),
        }
    }
}
