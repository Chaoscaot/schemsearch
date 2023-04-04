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
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_this_or_that::as_i64;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SchematicVersioned {
    V1,
    V2(SpongeV2Schematic),
    V3(SpongeV3Schematic),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeV2Schematic {
    pub version: i32,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpongeV3Schematic {
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

fn read_blockdata<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<i8> = Deserialize::deserialize(deserializer)?;
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

impl SchematicVersioned {
    pub fn load_data<R>(data: R) -> Result<SchematicVersioned, Error> where R: Read {
        let schematic: SchematicVersioned = from_gzip_reader(data)?;
        Ok(schematic)
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
