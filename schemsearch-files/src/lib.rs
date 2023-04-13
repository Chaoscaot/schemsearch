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

use std::io::Read;
use std::path::PathBuf;
use nbt::{Error, from_gzip_reader, Map, Value};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SchematicRaw {
    version: i32,
    #[serde(flatten)]
    data: Map<String, Value>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged, rename_all = "PascalCase")]
pub enum SchematicVersioned {
    V1(SpongeV1Schematic),
    V2(SpongeV2Schematic),
    V3(SpongeV3Schematic),
}

impl SchematicVersioned {
    #[inline]
    pub fn get_width(&self) -> u16 {
        return match self {
            SchematicVersioned::V1(schematic) => schematic.width,
            SchematicVersioned::V2(schematic) => schematic.width,
            SchematicVersioned::V3(schematic) => schematic.width,
        };
    }

    #[inline]
    pub fn get_height(&self) -> u16 {
        return match self {
            SchematicVersioned::V1(schematic) => schematic.height,
            SchematicVersioned::V2(schematic) => schematic.height,
            SchematicVersioned::V3(schematic) => schematic.height,
        };
    }

    #[inline]
    pub fn get_length(&self) -> u16 {
        return match self {
            SchematicVersioned::V1(schematic) => schematic.length,
            SchematicVersioned::V2(schematic) => schematic.length,
            SchematicVersioned::V3(schematic) => schematic.length,
        };
    }

    #[inline]
    pub fn get_palette_max(&self) -> i32 {
        return match self {
            SchematicVersioned::V1(schematic) => schematic.palette_max,
            SchematicVersioned::V2(schematic) => schematic.palette_max,
            SchematicVersioned::V3(schematic) => schematic.blocks.palette.len() as i32,
        };
    }

    #[inline]
    pub fn get_palette(&self) -> &Map<String, i32> {
        return match self {
            SchematicVersioned::V1(schematic) => &schematic.palette,
            SchematicVersioned::V2(schematic) => &schematic.palette,
            SchematicVersioned::V3(schematic) => &schematic.blocks.palette,
        };
    }

    #[inline]
    pub fn get_block_data(&self) -> &Vec<i32> {
        return match self {
            SchematicVersioned::V1(schematic) => &schematic.block_data,
            SchematicVersioned::V2(schematic) => &schematic.block_data,
            SchematicVersioned::V3(schematic) => &schematic.blocks.block_data,
        };
    }

    #[inline]
    pub fn get_block_entities(&self) -> &Vec<BlockEntity> {
        return match self {
            SchematicVersioned::V1(schematic) => &schematic.tile_entities,
            SchematicVersioned::V2(schematic) => &schematic.block_entities,
            SchematicVersioned::V3(schematic) => &schematic.blocks.block_entities,
        };
    }
}

impl From<SchematicRaw> for SchematicVersioned {
    fn from(value: SchematicRaw) -> Self {
        match value.version {
            1 => {
                let schematic: SpongeV1Schematic = serde_json::from_value(serde_json::to_value(value.data).unwrap()).unwrap();
                return SchematicVersioned::V1(schematic);
            },
            2 => {
                let schematic: SpongeV2Schematic = serde_json::from_value(serde_json::to_value(value.data).unwrap()).unwrap();
                return SchematicVersioned::V2(schematic);
            },
            3 => {
                let schematic: SpongeV3Schematic = serde_json::from_value(serde_json::to_value(value.data).unwrap()).unwrap();
                return SchematicVersioned::V3(schematic);
            }
            _ => panic!("Unknown Schematic Version: {}", value.version),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeV1Schematic {
    pub metadata: Map<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub palette_max: i32,
    pub palette: Map<String, i32>,
    #[serde(deserialize_with = "read_blockdata")]
    pub block_data: Vec<i32>,
    pub tile_entities: Vec<BlockEntity>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeV2Schematic {
    pub data_version: i32,
    pub metadata: Map<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub palette_max: i32,
    pub palette: Map<String, i32>,
    #[serde(deserialize_with = "read_blockdata")]
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Option<Vec<Entity>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeV3Schematic {
    pub data_version: i32,
    pub metadata: Map<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub blocks: BlockContainer,
    pub entities: Option<Vec<Entity>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BlockContainer {
    pub palette: Map<String, i32>,
    #[serde(deserialize_with = "read_blockdata")]
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
}

fn read_blockdata<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<i8> = Deserialize::deserialize(deserializer)?;
    Ok(read_varint_array(&s))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntity {
    pub id: String,
    pub pos: [i32; 3],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BlockEntityV3 {
    pub id: String,
    pub pos: [i32; 3],
    pub data: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Entity {
    pub id: String,
    pub pos: [i32; 3],
}

impl SchematicVersioned {
    pub fn load_data<R>(data: R) -> Result<SchematicVersioned, Error> where R: Read {
        let raw: SchematicRaw = from_gzip_reader(data)?;
        Ok(SchematicVersioned::from(raw))
    }

    pub fn load(path: &PathBuf) -> Result<SchematicVersioned, Error> {
        let file = std::fs::File::open(path)?;
        Self::load_data(file)
    }
}

pub fn read_varint_array(read: &Vec<i8>) -> Vec<i32> {
    let mut data = Vec::new();
    let mut value: i32 = 0;
    let mut position = 0;
    let mut current_byte;
    let mut cursor = 0;
    loop {
        match read.get(cursor) {
            Some(byte) => { current_byte = *byte as u8; cursor += 1; },
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
