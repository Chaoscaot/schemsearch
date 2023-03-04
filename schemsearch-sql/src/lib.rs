use sqlx::{Executor, MySql, Pool, Row};
use sqlx::mysql::MySqlPoolOptions;

mod properties;

pub struct SchematicNode {
    pub id: i32,
    pub name: String,
    pub node_owner: i32,

}

pub async fn get_connection() -> Pool<MySql> {
    let properties = properties::load_mysql_properties();
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&format!("mysql://{}:{}@{}/{}", properties.user, properties.password, properties.host, properties.database))
        .await.expect("Failed to connect to database")
}

pub async fn load_schematics(conn: &mut Pool<MySql>) -> Vec<SchematicNode> {
    let mut schematics = Vec::new();
    let rows = conn.fetch_all("SELECT id, name, node_owner FROM schematics").await.expect("Failed to fetch schematics");
    for row in rows {
        schematics.push(SchematicNode {
            id: row.get(0),
            name: row.get(1),
            node_owner: row.get(2),
        });
    }
    schematics
}

pub async fn load_data(conn: &mut Pool<MySql>, schematic: &SchematicNode) -> Vec<u8> {
    let rows = conn.fetch_one(sqlx::query("SELECT SchemData FROM NodeData WHERE NodeId = {}").bind(schematic.id)).await.expect("Failed to fetch schematic data");
    rows.get(0)
}

