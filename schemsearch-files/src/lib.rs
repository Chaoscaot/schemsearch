use std::path::Path;
use nbt::{Map, Value};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Schematic {
    #[serde(rename = "Version")]
    pub version: i32,
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "Metadata")]
    pub metadata: Map<String, Value>,
    #[serde(rename = "Width")]
    pub width: u16,
    #[serde(rename = "Height")]
    pub height: u16,
    #[serde(rename = "Length")]
    pub length: u16,
    #[serde(rename = "Offset")]
    pub offset: [i32; 3],
    #[serde(rename = "PaletteMax")]
    pub palette_max: i32,
    #[serde(rename = "Palette")]
    pub palette: Map<String, i32>,
    #[serde(rename = "BlockData", deserialize_with = "read_blockdata")]
    pub block_data: Vec<i32>,
    #[serde(rename = "BlockEntities")]
    pub block_entities: Vec<BlockEntity>,
    #[serde(rename = "Entities")]
    pub entities: Option<Vec<Entity>>,
}

fn read_blockdata<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<u8> = Deserialize::deserialize(deserializer)?;
    Ok(read_varint_array(&s))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockEntity {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Pos")]
    pub pos: [i32; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entity {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Pos")]
    pub pos: [i32; 3],
}

impl Schematic {
    pub fn load(path: &Path) -> Result<Schematic, String> {
        let file = match std::fs::File::open(path) {
            Ok(x) => x,
            Err(_) => return Err(format!("Failed to open file: {}", path.display()))
        };
        let schematic: Schematic = match nbt::from_gzip_reader(file) {
            Ok(schem) => schem,
            Err(e) => return Err(format!("Failed to parse schematic: {}", e))
        };
        Ok(schematic)
    }
}

pub fn read_varint_array(read: &Vec<u8>) -> Vec<i32> {
    let mut data = Vec::new();
    let mut value: i32 = 0;
    let mut position = 0;
    let mut current_byte;
    let mut cursor = 0;
    loop {
        match read.get(cursor) {
            Some(byte) => { current_byte = *byte; cursor += 1; },
            None => break,
        };

        value |= (((current_byte & 0x7F) as u32) << position) as i32;

        if(current_byte & 0x80) == 0 {
            data.push(value);
            value = 0;
            position = 0;
        } else {
            position += 7;

            if position > 32 {
                panic!("VarInt too big");
            }
        }
    }
    data
}
