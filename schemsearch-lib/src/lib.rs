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

pub mod pattern_mapper;

use serde::{Deserialize, Serialize};
use pattern_mapper::match_palette;
use schemsearch_files::Schematic;
use crate::pattern_mapper::match_palette_adapt;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SearchBehavior {
    pub ignore_block_data: bool,
    pub ignore_block_entities: bool,
    pub ignore_air: bool,
    pub air_as_any: bool,
    pub ignore_entities: bool,
    pub threshold: f32,
}

pub fn search(
    schem: Schematic,
    pattern_schem: &Schematic,
    search_behavior: SearchBehavior,
) -> Vec<Match> {
    if schem.width < pattern_schem.width || schem.height < pattern_schem.height || schem.length < pattern_schem.length {
        return vec![];
    }

    if pattern_schem.palette.len() > schem.palette.len() {
        return vec![];
    }

    let pattern_schem = match_palette(&schem, &pattern_schem, search_behavior.ignore_block_data);

    let mut matches: Vec<Match> = Vec::new();

    let pattern_data = pattern_schem.block_data.as_slice();

    let schem_data = if search_behavior.ignore_block_data {
        match_palette_adapt(&schem, &pattern_schem.palette, search_behavior.ignore_block_data)
    } else {
        schem.block_data
    };

    let schem_data = schem_data.as_slice();

    let air_id = if search_behavior.ignore_air || search_behavior.air_as_any { pattern_schem.palette.get("minecraft:air").unwrap_or(&-1) } else { &-1};

    let pattern_blocks = (pattern_schem.width * pattern_schem.height * pattern_schem.length) as f32;

    let pattern_width = pattern_schem.width as usize;
    let pattern_height = pattern_schem.height as usize;
    let pattern_length = pattern_schem.length as usize;

    let schem_width = schem.width as usize;
    let schem_height = schem.height as usize;
    let schem_length = schem.length as usize;

    for y in 0..=schem_height - pattern_height {
        for z in 0..=schem_length - pattern_length {
            for x in 0..=schem_width - pattern_width {
                let mut matching = 0;
                for j in 0..pattern_height {
                    for k in 0..pattern_length {
                        for i in 0..pattern_width {
                            let index = (x + i) + schem_width * ((z + k) + (y + j) * schem_length);
                            let pattern_index = i + pattern_width * (k + j * pattern_length);
                            let data = unsafe {schem_data.get_unchecked(index) };
                            let pattern_data = unsafe { pattern_data.get_unchecked(pattern_index) };
                            if *data == *pattern_data || (search_behavior.ignore_air && *data == *air_id) || (search_behavior.air_as_any && *pattern_data == *air_id) {
                                matching += 1;
                            }
                        }
                    }
                }
                let matching_percent = matching as f32 / pattern_blocks;
                if matching_percent >= search_behavior.threshold {
                    matches.push(Match {
                        x: x as u16,
                        y: y as u16,
                        z: z as u16,
                        percent: matching_percent,
                    });
                }
            }
        }
    }

    return matches;
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Match {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub percent: f32,
}

impl Default for Match {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            percent: 0.0,
        }
    }
}

#[inline]
pub fn normalize_data(data: &str, ignore_data: bool) -> &str {
    if ignore_data {
        data.split('[').next().unwrap()
    } else {
        data
    }
}

pub fn parse_schematic(data: &Vec<u8>) -> Schematic {
    if data[0] == 0x1f && data[1] == 0x8b {
        // gzip
        nbt::from_gzip_reader(data.as_slice()).unwrap()
    } else {
        // uncompressed
        nbt::from_reader(data.as_slice()).unwrap()
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use schemsearch_files::Schematic;
    use crate::pattern_mapper::strip_data;
    use super::*;

    #[test]
    fn read_schematic() {
        let schematic = Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
    }

    #[test]
    fn test_parse_function() {
        let file = std::fs::File::open("../tests/simple.schem").expect("Failed to open file");
        let schematic: Schematic = parse_schematic(&std::io::Read::bytes(file).map(|b| b.unwrap()).collect());
        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
    }

    #[test]
    fn test_strip_schem() {
        let schematic = Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let stripped = strip_data(&schematic);

        assert_eq!(stripped.palette.keys().any(|k| k.contains('[')), false);
    }

    #[test]
    fn test_match_palette() {
        let schematic = Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, true);
    }

    #[test]
    fn test_match_palette_ignore_data() {
        let schematic = Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, false);
    }

    #[test]
    pub fn test_big_search() {
        let schematic = Schematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = search(schematic, &endstone, SearchBehavior {
            ignore_block_data: true,
            ignore_block_entities: true,
            ignore_entities: true,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9
        });
    }

    #[test]
    pub fn test_search() {
        let schematic = Schematic::load(&PathBuf::from("../tests/Random.schem")).unwrap();
        let pattern = Schematic::load(&PathBuf::from("../tests/Pattern.schem")).unwrap();

        let matches = search(schematic, &pattern, SearchBehavior {
            ignore_block_data: true,
            ignore_block_entities: true,
            ignore_entities: true,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9
        });

        println!("{:?}", matches);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], (1, 0, 3, 1.0));
    }

    #[test]
    pub fn test_search_ws() {
        let schematic = Schematic::load(&PathBuf::from("../tests/warships/GreyFly-by-Bosslar.schem")).unwrap();
        let pattern = Schematic::load(&PathBuf::from("../tests/gray_castle_complex.schem")).unwrap();

        let matches = search(schematic, &pattern, SearchBehavior {
            ignore_block_data: false,
            ignore_block_entities: false,
            ignore_entities: false,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9
        });

        println!("{:?}", matches);
        assert_eq!(matches.len(), 1);
    }
}
