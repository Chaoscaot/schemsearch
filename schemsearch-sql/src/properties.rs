use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

pub(crate) struct SqlProperties {
    pub(crate) host: String,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
}

pub(crate) fn load_mysql_properties() -> SqlProperties {
    let content = read_to_string(Path::new(&std::env::var("HOME").unwrap()).join("mysql.properties")).expect("Failed to read mysql.properties");
    let mut properties: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        let split: Vec<&str> = line.split('=').collect();
        properties.insert(split[0].to_string(), split[1].to_string());
    }

    SqlProperties {
        host: properties.get("host").unwrap().to_string(),
        user: properties.get("user").unwrap().to_string(),
        password: properties.get("password").unwrap().to_string(),
        database: properties.get("database").unwrap().to_string(),
    }
}