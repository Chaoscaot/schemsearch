/*
 * Copyright (C) 2023  Chaoscaot
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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