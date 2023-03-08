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

