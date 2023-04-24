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

use std::collections::HashMap;
use nbt::CompoundTag;
use schemsearch_files::SpongeSchematic;
use crate::normalize_data;

fn create_reverse_palette(schem: &SpongeSchematic) -> Vec<&str> {
    let mut reverse_palette = Vec::with_capacity(schem.palette_max as usize);
    (0..schem.palette_max).for_each(|_| reverse_palette.push(""));
    for (key, value) in schem.palette.iter() {
        reverse_palette[*value as usize] = key;
    }
    reverse_palette
}

pub fn strip_data(schem: &SpongeSchematic) -> SpongeSchematic {
    let mut data: Vec<i32> = Vec::new();

    let mut palette: HashMap<String, i32> = HashMap::new();
    let mut palette_max: i32 = 0;
    let reverse_palette = create_reverse_palette(schem);

    for block in schem.block_data.iter() {
        let block_name = reverse_palette[*block as usize].clone();
        let block_name = block_name.split('[').next().unwrap().to_string();

        let entry = palette.entry(block_name).or_insert_with(|| {
            let value = palette_max;
            palette_max += 1;
            value
        });
        data.push(*entry);
    }

    SpongeSchematic {
        data_version: 1,
        palette,
        palette_max,
        block_data: data,
        block_entities: schem.block_entities.clone(),
        height: schem.height,
        length: schem.length,
        width: schem.width,
        metadata: CompoundTag::new(),
        offset: [0; 3],
        entities: None,
    }


}

pub fn match_palette_adapt(schem: &SpongeSchematic, matching_palette: &HashMap<String, i32>, ignore_data: bool) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    let reverse_palette = create_reverse_palette(schem);

    for x in schem.block_data.iter() {
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
    schem: &SpongeSchematic,
    pattern: &SpongeSchematic,
    ignore_data: bool,
) -> SpongeSchematic {
    if ignore_data {
        match_palette_internal(&strip_data(schem), &strip_data(pattern), ignore_data)
    } else {
        match_palette_internal(schem, pattern, ignore_data)
    }
}

fn match_palette_internal(
    schem: &SpongeSchematic,
    pattern: &SpongeSchematic,
    ignore_data: bool,
) -> SpongeSchematic {
    let data_pattern: Vec<i32> = match_palette_adapt(&pattern, &schem.palette, ignore_data);

    SpongeSchematic {
        data_version: 0,
        palette: schem.palette.clone(),
        palette_max: schem.palette_max,
        block_data: data_pattern,
        block_entities: pattern.block_entities.clone(),
        height: pattern.height,
        length: pattern.length,
        width: pattern.width,
        metadata: CompoundTag::new(),
        offset: [0; 3],
        entities: None,
    }
}