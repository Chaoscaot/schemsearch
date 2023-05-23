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
pub struct SpongeSchematic {
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

impl SpongeSchematic {
    pub fn load_data<R>(data: &mut R) -> Result<SpongeSchematic, String> where R: Read {
        let nbt: CompoundTag = nbt::decode::read_gzip_compound_tag(data).map_err(|e| e.to_string())?;
        let version = nbt.get_i32("Version").unwrap_or_else(|_| {
            return if nbt.contains_key("Blocks") {
                3
            } else if nbt.contains_key("BlockEntities") {
                2
            } else if nbt.contains_key("TileEntities") {
                1
            } else {
                -1
            };
        });

        match version {
            1 => SpongeSchematic::from_nbt_1(nbt),
            2 => SpongeSchematic::from_nbt_2(nbt),
            3 => SpongeSchematic::from_nbt_3(nbt),
            _ => Err("Invalid schematic: Unknown Version".to_string()),
        }
    }

    pub fn load(path: &PathBuf) -> Result<SpongeSchematic, String> {
        let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        Self::load_data(&mut file)
    }

    pub fn from_nbt_1(nbt: CompoundTag) -> Result<Self, String> {
        Ok(Self {
            data_version: 0,
            metadata: nbt.get_compound_tag("Metadata").map_err(|e| e.to_string())?.clone(),
            width: nbt.get_i16("Width").map_err(|e| e.to_string())? as u16,
            height: nbt.get_i16("Height").map_err(|e| e.to_string())? as u16,
            length: nbt.get_i16("Length").map_err(|e| e.to_string())? as u16,
            offset: read_offset(nbt.get_i32_vec("Offset").map_err(|e| e.to_string())?)?,
            palette_max: nbt.get_i32("PaletteMax").map_err(|e| e.to_string())?,
            palette: read_palette(nbt.get_compound_tag("Palette").map_err(|e| e.to_string())?),
            block_data: read_blocks(nbt.get_i8_vec("BlockData").map_err(|e| e.to_string())?),
            block_entities: read_tile_entities(nbt.get_compound_tag_vec("TileEntities").unwrap_or_else(|_| vec![]))?,
            entities: None,
        })
    }

    pub fn from_nbt_2(nbt: CompoundTag) -> Result<Self, String> {
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
            block_entities: read_tile_entities(nbt.get_compound_tag_vec("BlockEntities").unwrap_or_else(|_| vec![]))?,
            entities: None,
        })
    }

    pub fn from_nbt_3(nbt: CompoundTag) -> Result<Self, String> {
        let blocks = nbt.get_compound_tag("Blocks").map_err(|e| e.to_string())?;
        Ok(Self{
            data_version: nbt.get_i32("DataVersion").map_err(|e| e.to_string())?,
            metadata: nbt.get_compound_tag("Metadata").map_err(|e| e.to_string())?.clone(),
            width: nbt.get_i16("Width").map_err(|e| e.to_string())? as u16,
            height: nbt.get_i16("Height").map_err(|e| e.to_string())? as u16,
            length: nbt.get_i16("Length").map_err(|e| e.to_string())? as u16,
            offset: read_offset(nbt.get_i32_vec("Offset").map_err(|e| e.to_string())?)?,
            palette_max: compute_palette_max(blocks.get_compound_tag("Palette").map_err(|e| e.to_string())?),
            palette: read_palette(blocks.get_compound_tag("Palette").map_err(|e| e.to_string())?),
            block_data: read_blocks(blocks.get_i8_vec("BlockData").map_err(|e| e.to_string())?),
            block_entities: read_tile_entities(blocks.get_compound_tag_vec("BlockEntities").unwrap_or_else(|_| vec![]))?,
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
fn compute_palette_max(palette: &CompoundTag) -> i32 {
    palette.iter().map(|(_, v)| v).filter_map(|v| match v {
        Tag::Int(n) => Some(*n),
        _ => None,
    }).max().unwrap_or(0)
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
