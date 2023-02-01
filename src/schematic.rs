use std::path::Path;
use nbt::{Map, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Schematic {
    #[serde(rename = "Version")]
    pub(crate) version: i32,
    #[serde(rename = "DataVersion")]
    pub(crate) data_version: i32,
    #[serde(rename = "Metadata")]
    pub(crate) metadata: Map<String, Value>,
    #[serde(rename = "Width")]
    pub(crate) width: u16,
    #[serde(rename = "Height")]
    pub(crate) height: u16,
    #[serde(rename = "Length")]
    pub(crate) length: u16,
    #[serde(rename = "Offset")]
    pub(crate) offset: [i32; 3],
    #[serde(rename = "PaletteMax")]
    pub(crate) palette_max: i32,
    #[serde(rename = "Palette")]
    pub(crate) palette: Map<String, i32>,
    #[serde(rename = "BlockData")]
    pub(crate) block_data: Vec<i32>,
    #[serde(rename = "BlockEntities")]
    pub(crate) block_entities: Vec<BlockEntity>,
    #[serde(rename = "Entities")]
    pub(crate) entities: Option<Vec<Entity>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct BlockEntity {
    #[serde(rename = "Id")]
    pub(crate) id: String,
    #[serde(rename = "Pos")]
    pub(crate) pos: [i32; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Entity {
    #[serde(rename = "Id")]
    pub(crate) id: String,
    #[serde(rename = "Pos")]
    pub(crate) pos: [i32; 3],
}

impl Schematic {
    pub(crate) fn load(path: &Path) -> Schematic {
        let file = std::fs::File::open(path).expect("Failed to open file");
        let schematic: Schematic = match nbt::from_gzip_reader(file) {
            Ok(schem) => schem,
            Err(e) => panic!("Failed to parse schematic: {}", e),
        };
        schematic
    }
}
