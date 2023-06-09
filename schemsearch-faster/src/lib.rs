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
use schemsearch_files::SpongeV2Schematic;

pub fn convert_to_search_space(schem: &SpongeV2Schematic, palette: &Vec<String>) -> Vec<Vec<u8>> {
    let mut data: Vec<Vec<u8>> = Vec::with_capacity(palette.len());
    let block_data = &schem.block_data;
    for name in palette {
        let mut output: Vec<u8> = Vec::with_capacity(block_data.len());
        for block in block_data.iter() {
            if schem.palette.get(name).unwrap_or(&-1) == block {
                output.push(1);
            } else {
                output.push(0);
            }
        }
        data.push(output);
    }
    data
}

pub fn unwrap_palette(palette: &Map<String, i32>) -> Vec<String> {
    let mut output: Vec<String> = Vec::with_capacity(palette.len());
    (0..palette.len()).for_each(|_| output.push(String::new()));
    for (key, id) in palette.iter() {
        output[*id as usize] = key.clone();
    }
    output
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use schemsearch_files::SpongeV2Schematic;
    use crate::{convert_to_search_space, unwrap_palette};

    //#[test]
    pub fn test() {
        let schematic = SpongeV2Schematic::load(&PathBuf::from("../tests/Pattern.schem")).unwrap();
        dbg!(convert_to_search_space(&schematic, &unwrap_palette(&schematic.palette)));
    }

    //#[test]
    pub fn test_2() {
        let schematic = SpongeV2Schematic::load(&PathBuf::from("../tests/Pattern.schem")).unwrap();
        let schematic2 = SpongeV2Schematic::load(&PathBuf::from("../tests/Random.schem")).unwrap();
        println!("{:?}", convert_to_search_space(&schematic2, &unwrap_palette(&schematic.palette)));
    }

    //#[test]
    pub fn test_big() {
        let schematic = SpongeV2Schematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();
        let schematic2 = SpongeV2Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let _ = convert_to_search_space(&schematic2, &unwrap_palette(&schematic.palette));
    }
}