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
use nbt::{CompoundTag, Tag};

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
    pub metadata: CompoundTag,
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
    pub metadata: CompoundTag,
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
    pub metadata: CompoundTag,
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
    pub data: HashMap<String, Tag>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub pos: [i32; 3],
}

impl SchematicVersioned {
    pub fn load_data<R>(data: &mut R) -> Result<SchematicVersioned, String> where R: Read {
        let nbt: CompoundTag = nbt::decode::read_gzip_compound_tag(data).map_err(|e| e.to_string())?;
        let version = nbt.get_i32("Version").map_err(|e| e.to_string())?;

        match version {
            1 => Ok(SchematicVersioned::V1(SpongeV1Schematic::from_nbt(nbt)?)),
            2 => Ok(SchematicVersioned::V2(SpongeV2Schematic::from_nbt(nbt)?)),
            3 => Ok(SchematicVersioned::V3(SpongeV3Schematic::from_nbt(nbt)?)),
            _ => Err("Invalid schematic: Unknown Version".to_string()),
        }
    }

    pub fn load(path: &PathBuf) -> Result<SchematicVersioned, String> {
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        Self::load_data(&mut file)
    }
}

impl SpongeV1Schematic {
    pub fn from_nbt(nbt: CompoundTag) -> Result<Self, String> {
        Ok(Self {
            metadata: nbt.get_compound_tag("Metadata").map_err(|e| e.to_string())?.clone(),
            width: nbt.get_i16("Width").map_err(|e| e.to_string())? as u16,
            height: nbt.get_i16("Height").map_err(|e| e.to_string())? as u16,
            length: nbt.get_i16("Length").map_err(|e| e.to_string())? as u16,
            offset: read_offset(nbt.get_i32_vec("Offset").map_err(|e| e.to_string())?)?,
            palette_max: nbt.get_i32("PaletteMax").map_err(|e| e.to_string())?,
            palette: read_palette(nbt.get_compound_tag("Palette").map_err(|e| e.to_string())?),
            block_data: read_blocks(nbt.get_i8_vec("BlockData").map_err(|e| e.to_string())?),
            tile_entities: read_tile_entities(nbt.get_compound_tag_vec("TileEntities").map_err(|e| e.to_string())?)?,
        })
    }
}

impl SpongeV2Schematic {
    pub fn from_nbt(nbt: CompoundTag) -> Result<Self, String> {
        Ok(Self{
            data_version: nbt.get_i32("DataVersion").map_err(|e| e.to_string())?,
            metadata: nbt.get_compound_tag("Metadata").map_err(|e| e.to_string())?.clone(),
            width: nbt.get_i16("Width").map_err(|e| e.to_string())? as u16,
            height: nbt.get_i16("Height").map_err(|e| e.to_string())? as u16,
            length: nbt.get_i16("Length").map_err(|e| e.to_string())? as u16,
            offset: read_offset(nbt.get_i32_vec("Offset").map_err(|e| e.to_string())?)?,
            palette_max: nbt.get_i32("PaletteMax").map_err(|e| e.to_string())?,
            palette: read_palette(nbt.get_compound_tag("Palette").map_err(|e| e.to_string())?),
            block_data: read_blocks(nbt.get_i8_vec("BlockData").map_err(|e| e.to_string())?),
            block_entities: read_tile_entities(nbt.get_compound_tag_vec("BlockEntities").map_err(|e| e.to_string())?)?,
            entities: None,
        })
    }
}

impl SpongeV3Schematic {
    pub fn from_nbt(nbt: CompoundTag) -> Result<Self, String> {
        let blocks = nbt.get_compound_tag("Blocks").map_err(|e| e.to_string())?;
        Ok(Self{
            data_version: nbt.get_i32("DataVersion").map_err(|e| e.to_string())?,
            metadata: nbt.get_compound_tag("Metadata").map_err(|e| e.to_string())?.clone(),
            width: nbt.get_i16("Width").map_err(|e| e.to_string())? as u16,
            height: nbt.get_i16("Height").map_err(|e| e.to_string())? as u16,
            length: nbt.get_i16("Length").map_err(|e| e.to_string())? as u16,
            offset: read_offset(nbt.get_i32_vec("Offset").map_err(|e| e.to_string())?)?,
            blocks: BlockContainer {
                palette: read_palette(blocks.get_compound_tag("Palette").map_err(|e| e.to_string())?),
                block_data: read_blocks(blocks.get_i8_vec("BlockData").map_err(|e| e.to_string())?),
                block_entities: read_tile_entities(blocks.get_compound_tag_vec("BlockEntities").map_err(|e| e.to_string())?)?,
            },
            entities: None,
        })
    }
}

fn read_tile_entities(tag: Vec<&CompoundTag>) -> Result<Vec<BlockEntity>, String> {
let mut tile_entities = Vec::new();
    for t in tag {
        tile_entities.push(BlockEntity {
            id: t.get_str("Id").map_err(|e| e.to_string())?.to_string(),
            pos: read_offset(t.get("Pos").map_err(|e| e.to_string())?)?,
        });
    }
    Ok(tile_entities)
}

#[inline]
fn read_offset(offset: &Vec<i32>) -> Result<[i32; 3], String> {
    match offset.len() {
        3 => Ok([offset[0], offset[1], offset[2]]),
        _ => Err("Invalid schematic: read_offset wrong length".to_string()),
    }
}

#[inline]
fn read_palette(p: &CompoundTag) -> HashMap<String, i32> {
    let mut palette = HashMap::new();
    for (key, value) in p.iter() {
        match value {
            Tag::Int(n) => { palette.insert(key.clone(), *n); },
            _ => {},
        };
    }
    palette
}

#[inline]
fn read_blocks(blockdata: &Vec<i8>) -> Vec<i32> {
    read_varint_array(blockdata)
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
