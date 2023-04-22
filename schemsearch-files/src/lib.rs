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

use std::collections::hash_map::HashMap;
use std::io::Read;
use std::path::PathBuf;
use nbt::Value;

#[derive(Clone, Debug)]
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
    pub fn get_palette(&self) -> &HashMap<String, i32> {
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

#[derive(Clone, Debug)]
pub struct SpongeV1Schematic {
    pub metadata: HashMap<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub palette_max: i32,
    pub palette: HashMap<String, i32>,
    pub block_data: Vec<i32>,
    pub tile_entities: Vec<BlockEntity>,
}

#[derive(Clone, Debug)]
pub struct SpongeV2Schematic {
    pub data_version: i32,
    pub metadata: HashMap<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub palette_max: i32,
    pub palette: HashMap<String, i32>,
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
    pub entities: Option<Vec<Entity>>,
}

#[derive(Clone, Debug)]
pub struct SpongeV3Schematic {
    pub data_version: i32,
    pub metadata: HashMap<String, Value>,
    pub width: u16,
    pub height: u16,
    pub length: u16,
    pub offset: [i32; 3],
    pub blocks: BlockContainer,
    pub entities: Option<Vec<Entity>>,
}

#[derive(Clone, Debug)]
pub struct BlockContainer {
    pub palette: HashMap<String, i32>,
    pub block_data: Vec<i32>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Debug, Clone)]
pub struct BlockEntity {
    pub id: String,
    pub pos: [i32; 3],
}

#[derive(Debug, Clone)]
pub struct BlockEntityV3 {
    pub id: String,
    pub pos: [i32; 3],
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub pos: [i32; 3],
}

impl SchematicVersioned {
    pub fn load_data<R>(data: R) -> Result<SchematicVersioned, String> where R: Read {
        let nbt: HashMap<String, Value> = nbt::de::from_gzip_reader(data).map_err(|e| e.to_string())?;
        let version = match nbt.get("Version") {
            Some(version) => match version {
                Value::Short(n) => *n as i32,
                Value::Byte(n) => *n as i32,
                Value::Int(n) => *n,
                _ => return Err("Invalid schematic: Wrong Version Type".to_string()),
            },
            None => return Err("Invalid schematic: Version not Found".to_string()),
        };

        match version {
            1 => Ok(SchematicVersioned::V1(SpongeV1Schematic::from_nbt(nbt)?)),
            2 => Ok(SchematicVersioned::V2(SpongeV2Schematic::from_nbt(nbt)?)),
            3 => Ok(SchematicVersioned::V3(SpongeV3Schematic::from_nbt(nbt)?)),
            _ => Err("Invalid schematic: Unknown Version".to_string()),
        }
    }

    pub fn load(path: &PathBuf) -> Result<SchematicVersioned, String> {
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        Self::load_data(file)
    }
}

impl SpongeV1Schematic {
    pub fn from_nbt(nbt: HashMap<String, Value>) -> Result<Self, String> {
        Ok(Self {
            metadata: match nbt.get("Metadata").ok_or("Invalid schematic: Metadata not found".to_string())? {
                Value::Compound(metadata) => metadata.clone(),
                _ => return Err("Invalid schematic: Metadata Wrong Type".to_string()),
            },
            width: match nbt.get("Width").ok_or("Invalid schematic: Width not found".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Width Wrong Type".to_string()),
            },
            height: match nbt.get("Height").ok_or("Invalid schematic: Height not found".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Height Wrong Type".to_string()),
            },
            length: match nbt.get("Length").ok_or("Invalid schematic: Length not found".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Length Wrong Type".to_string()),
            },
            offset: read_offset(nbt.get("Offset"))?,
            palette_max: match nbt.get("PaletteMax").ok_or("Invalid schematic: PaletteMax not found".to_string())? {
                Value::Int(p) => *p,
                _ => return Err("Invalid schematic: PaletteMax Wrong Type".to_string()),
            },
            palette: read_palette(nbt.get("Palette"))?,
            block_data: read_blocks(nbt.get("BlockData"))?,
            tile_entities: read_tile_entities(nbt.get("TileEntities"))?,
        })
    }
}

impl SpongeV2Schematic {
    pub fn from_nbt(nbt: HashMap<String, Value>) -> Result<Self, String> {
        Ok(Self{
            data_version: match nbt.get("DataVersion").ok_or("Invalid schematic: DataVersion Missing".to_string())? {
                Value::Short(n) => *n as i32,
                Value::Byte(n) => *n as i32,
                Value::Int(n) => *n,
                _ => return Err("Invalid schematic: DataVersion Wrong Type".to_string()),
            },
            metadata: match nbt.get("Metadata").ok_or("Invalid schematic".to_string())? {
                Value::Compound(m) => m.clone(),
                _ => return Err("Invalid schematic: Metadata Wrong Type".to_string()),
            },
            width: match nbt.get("Width").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Width Wrong Type".to_string()),
            },
            height: match nbt.get("Height").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Height Wrong Type".to_string()),
            },
            length: match nbt.get("Length").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic: Length Wrong Type".to_string()),
            },
            offset: read_offset(nbt.get("Offset"))?,
            palette_max: match nbt.get("PaletteMax").ok_or("Invalid schematic: PaletteMax Missing".to_string())? {
                Value::Short(n) => *n as i32,
                Value::Byte(n) => *n as i32,
                Value::Int(n) => *n,
                _ => return Err("Invalid schematic: PaletteMax Invalid Type".to_string()),
            },
            palette: read_palette(nbt.get("Palette"))?,
            block_data: read_blocks(nbt.get("BlockData"))?,
            block_entities: read_tile_entities(nbt.get("BlockEntities"))?,
            entities: None,
        })
    }
}

impl SpongeV3Schematic {
    pub fn from_nbt(nbt: HashMap<String, Value>) -> Result<Self, String> {
        Ok(Self{
            data_version: match nbt.get("DataVersion").ok_or("Invalid schematic".to_string())? {
                Value::Int(d) => *d,
                _ => return Err("Invalid schematic".to_string()),
            },
            metadata: match nbt.get("Metadata").ok_or("Invalid schematic".to_string())? {
                Value::Compound(m) => m.clone(),
                _ => return Err("Invalid schematic".to_string()),
            },
            width: match nbt.get("Width").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic".to_string()),
            },
            height: match nbt.get("Height").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic".to_string()),
            },
            length: match nbt.get("Length").ok_or("Invalid schematic".to_string())? {
                Value::Short(n) => *n as u16,
                Value::Byte(n) => *n as u16,
                _ => return Err("Invalid schematic".to_string()),
            },
            offset: read_offset(nbt.get("Offset"))?,
            blocks: match nbt.get("Blocks").ok_or("Invalid schematic".to_string())? {
                Value::Compound(b) => {
                    BlockContainer {
                        palette: read_palette(b.get("Palette"))?,
                        block_data: read_blocks(b.get("BlockData"))?,
                        block_entities: read_tile_entities(b.get("BlockEntities"))?,
                    }
                }
                _ => return Err("Invalid schematic".to_string()),
            },
            entities: None,
        })
    }
}

fn read_tile_entities(tag: Option<&Value>) -> Result<Vec<BlockEntity>, String> {
    match tag.ok_or("Invalid schematic: read_tile_entities not found".to_string())? {
        Value::List(t) => {
            let mut tile_entities = Vec::new();
            for te in t.iter() {
                match te {
                    Value::Compound(te) => {
                        let id = match te.get("Id") {
                            None => return Err("Invalid schematic: Id Not Found".to_string()),
                            Some(id) => match id {
                                Value::String(id) => id.clone(),
                                _ => return Err("Invalid schematic: Id Wrong Type".to_string()),
                            },
                        };
                        let pos = read_offset(te.get("Pos"))?;
                        tile_entities.push(BlockEntity { id, pos });
                    },
                    _ => return Err("Invalid schematic: te Wrong Type".to_string()),
                };
            }
            Ok(tile_entities)
        },
        Value::ByteArray(_) => Ok(vec![]),
        _ => return Err("Invalid schematic: te wrong type".to_string()),
    }
}

#[inline]
fn read_offset(offset: Option<&Value>) -> Result<[i32; 3], String> {
    match offset.ok_or("Invalid schematic: read_offset missing".to_string())? {
        Value::IntArray(o) => match o.len() {
            3 => Ok([o[0], o[1], o[2]]),
            _ => Err("Invalid schematic: Invalid IntArray".to_string()),
        },
        Value::ByteArray(o) => match o.len() {
            3 => Ok([o[0] as i32, o[1]  as i32, o[2] as i32]),
            _ => Err("Invalid schematic: Invalid byteArray".to_string()),
        },
        Value::List(l) => match l.len() {
            3 => {
                let mut offset = [0; 3];
                for (i, v) in l.iter().enumerate() {
                    match v {
                        Value::Int(n) => offset[i] = *n,
                        Value::Byte(n) => offset[i] = *n as i32,
                        Value::Short(n) => offset[i] = *n as i32,
                        _ => return Err("Invalid schematic: read_offset invalid Number".to_string()),
                    };
                }
                Ok(offset)
            },
            _ => Err("Invalid schematic: Invalid List".to_string()),
        }
        _ => Err("Invalid schematic: read_offset".to_string()),
    }
}

#[inline]
fn read_palette(palette: Option<&Value>) -> Result<HashMap<String, i32>, String> {
    match palette.ok_or("Invalid schematic: read_palette missing".to_string())? {
        Value::Compound(p) => {
            let mut palette = HashMap::new();
            for (k, v) in p.iter() {
                match v {
                    Value::Int(v) => { palette.insert(k.clone(), *v); },
                    Value::Byte(v) => { palette.insert(k.clone(), *v as i32); },
                    Value::Short(v) => { palette.insert(k.clone(), *v as i32); },
                    _ => return Err("Invalid schematic: read_palette invalid Number".to_string()),
                };
            }
            Ok(palette)
        },
        _ => Err("Invalid schematic: read_palette invalid Type".to_string()),
    }
}

#[inline]
fn read_blocks(blockdata: Option<&Value>) -> Result<Vec<i32>, String> {
    match blockdata.ok_or("Invalid schematic: BlockData not found".to_string())? {
        Value::ByteArray(b) => Ok(read_varint_array(b)),
        _ => Err("Invalid schematic: Invalid BlockData".to_string()),
    }
}

#[inline]
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
