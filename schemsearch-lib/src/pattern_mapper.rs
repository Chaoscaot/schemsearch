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

use nbt::Map;
use schemsearch_files::{SchematicVersioned, SpongeV2Schematic};
use crate::normalize_data;

fn create_reverse_palette(schem: &SchematicVersioned) -> Vec<&str> {
    let mut reverse_palette = Vec::with_capacity(schem.get_palette_max() as usize);
    (0..schem.get_palette_max()).for_each(|_| reverse_palette.push(""));
    for (key, value) in schem.get_palette().iter() {
        reverse_palette[*value as usize] = key;
    }
    reverse_palette
}

pub fn strip_data(schem: &SchematicVersioned) -> SchematicVersioned {
    let mut data: Vec<i32> = Vec::new();

    let mut palette: Map<String, i32> = Map::new();
    let mut palette_max: i32 = 0;
    let reverse_palette = create_reverse_palette(schem);

    for block in schem.get_block_data().iter() {
        let block_name = reverse_palette[*block as usize].clone();
        let block_name = block_name.split('[').next().unwrap().to_string();

        let entry = palette.entry(block_name).or_insert_with(|| {
            let value = palette_max;
            palette_max += 1;
            value
        });
        data.push(*entry);
    }

    SchematicVersioned::V2(SpongeV2Schematic {
        data_version: 1,
        palette,
        palette_max,
        block_data: data,
        block_entities: schem.get_block_entities().clone(),
        height: schem.get_height(),
        length: schem.get_length(),
        width: schem.get_width(),
        metadata: Map::new(),
        offset: [0; 3],
        entities: None,
    },)


}

pub fn match_palette_adapt(schem: &SchematicVersioned, matching_palette: &Map<String, i32>, ignore_data: bool) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    let reverse_palette = create_reverse_palette(schem);

    for x in schem.get_block_data() {
        let blockname = reverse_palette[*x as usize];
        let blockname = if ignore_data { normalize_data(blockname, ignore_data) } else { blockname };
        let block_id = match matching_palette.get(&*blockname) {
            None => -1,
            Some(x) => *x
        };
        data.push(block_id);
    }

    data
}

pub fn match_palette(
    schem: &SchematicVersioned,
    pattern: &SchematicVersioned,
    ignore_data: bool,
) -> SchematicVersioned {
    if ignore_data {
        match_palette_internal(&strip_data(schem), &strip_data(pattern), ignore_data)
    } else {
        match_palette_internal(schem, pattern, ignore_data)
    }
}

fn match_palette_internal(
    schem: &SchematicVersioned,
    pattern: &SchematicVersioned,
    ignore_data: bool,
) -> SchematicVersioned {
    let data_pattern: Vec<i32> = match_palette_adapt(&pattern, schem.get_palette(), ignore_data);

    SchematicVersioned::V2(SpongeV2Schematic {
        data_version: 0,
        palette: schem.get_palette().clone(),
        palette_max: schem.get_palette_max(),
        block_data: data_pattern,
        block_entities: pattern.get_block_entities().clone(),
        height: pattern.get_height(),
        length: pattern.get_length(),
        width: pattern.get_width(),
        metadata: Map::new(),
        offset: [0; 3],
        entities: None,
    })
}