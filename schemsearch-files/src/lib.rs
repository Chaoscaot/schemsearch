use std::path::Path;
use nbt::{Map, Value};
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "BlockData")]
    pub block_data: Vec<u8>,
    #[serde(rename = "BlockEntities")]
    pub block_entities: Vec<BlockEntity>,
    #[serde(rename = "Entities")]
    pub entities: Option<Vec<Entity>>,
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
    pub fn load(path: &Path) -> Schematic {
        let file = std::fs::File::open(path).expect("Failed to open file");
        let schematic: Schematic = match nbt::from_gzip_reader(file) {
            Ok(schem) => schem,
            Err(e) => panic!("Failed to parse schematic: {}", e),
        };
        schematic
    }

    pub fn read_blockdata(&self) -> Vec<i32> {
        read_varint_array(&self.block_data)
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

pub fn to_varint_array(data: &Vec<i32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for value in data {
        let mut value = *value as u32;
        'inner: loop {
            if (value & 0x80) == 0 {
                bytes.push(value as u8);
                break 'inner;
            }

            bytes.push((value & 0x7F) as u8 | 0x80);
            value >>= 7;
        }
    }
    bytes
}