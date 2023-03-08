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

use std::sync::Mutex;
use sqlx::{Executor, MySql, MySqlPool, Pool, Row};
use sqlx::mysql::MySqlConnectOptions;

mod properties;

static mut CONN: Mutex<Option<Pool<MySql>>> = Mutex::new(None);

pub struct SchematicNode {
    pub id: i32,
    pub name: String
}

pub async unsafe fn get_connection() {
    let mut conn = CONN.lock().unwrap();
    if conn.is_none() {
        let properties = properties::load_mysql_properties();
        let _ = conn.insert(MySqlPool::connect_with(
            MySqlConnectOptions::new()
                .host(properties.host.as_str())
                .port(3306)
                .username(properties.user.as_str())
                .password(properties.password.as_str())
                .database(properties.database.as_str())
        )
            .await.expect("Failed to connect to database"));
    }
}

pub async fn load_all_schematics() -> Vec<SchematicNode> {
    unsafe { get_connection().await; }
    let mut schematics = Vec::new();
    let rows = unsafe { &CONN }.lock().unwrap().as_mut().unwrap().fetch_all("SELECT SN.NodeId, SN.NodeName FROM NodeData ND INNER JOIN SchematicNode SN ON SN.NodeId = ND.NodeId WHERE NodeFormat = true").await.expect("Failed to fetch schematics");
    for row in rows {
        schematics.push(SchematicNode {
            id: row.get(0),
            name: row.get(1)
        });
    }
    schematics
}

pub async fn load_schemdata(id: i32) -> Vec<u8> {
    unsafe { get_connection().await; }
    let rows = unsafe { &CONN }.lock().unwrap().as_mut().unwrap().fetch_one(format!("SELECT SchemData FROM NodeData WHERE NodeId = {}", id).as_str()).await.expect("Failed to fetch schematics");
    rows.get(0)
}

