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
pub mod search;
pub mod nbt_search;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SearchBehavior {
    pub ignore_block_data: bool,
    pub ignore_block_entities: bool,
    pub ignore_air: bool,
    pub air_as_any: bool,
    pub ignore_entities: bool,
    pub threshold: f32,
    pub invalid_nbt: bool,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Match {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub percent: f32,
}

#[inline]
pub fn normalize_data(data: &str, ignore_data: bool) -> &str {
    if ignore_data {
        data.split('[').next().unwrap()
    } else {
        data
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use schemsearch_files::SpongeSchematic;
    use crate::pattern_mapper::{match_palette, strip_data};
    use crate::search::search;
    use super::*;

    #[test]
    fn read_schematic() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();

        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
    }

    #[test]
    fn test_parse_function() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();

        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
    }

    #[test]
    fn test_strip_schem() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let stripped = strip_data(&schematic);

        assert_eq!(stripped.palette.keys().any(|k| k.contains('[')), false);
    }

    #[test]
    fn test_match_palette() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = SpongeSchematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, true);
    }

    #[test]
    fn test_match_palette_ignore_data() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = SpongeSchematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, false);
    }

    #[test]
    pub fn test_big_search() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/simple.schem")).unwrap();
        let endstone = SpongeSchematic::load(&PathBuf::from("../tests/endstone.schem")).unwrap();

        let _ = search(schematic, &endstone, SearchBehavior {
            ignore_block_data: true,
            ignore_block_entities: true,
            ignore_entities: true,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9,
            invalid_nbt: false
        });
    }

    #[test]
    pub fn test_search() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/Random.schem")).unwrap();
        let pattern = SpongeSchematic::load(&PathBuf::from("../tests/Pattern.schem")).unwrap();

        let matches = search(schematic, &pattern, SearchBehavior {
            ignore_block_data: true,
            ignore_block_entities: true,
            ignore_entities: true,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9,
            invalid_nbt: false
        });

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].x, 1);
        assert_eq!(matches[0].y, 0);
        assert_eq!(matches[0].z, 3);
        assert_eq!(matches[0].percent, 1.0);
    }

    #[test]
    pub fn test_search_ws() {
        let schematic = SpongeSchematic::load(&PathBuf::from("../tests/warships/GreyFly-by-Bosslar.schem")).unwrap();
        let pattern = SpongeSchematic::load(&PathBuf::from("../tests/gray_castle_complex.schem")).unwrap();

        let matches = search(schematic, &pattern, SearchBehavior {
            ignore_block_data: false,
            ignore_block_entities: false,
            ignore_entities: false,
            ignore_air: false,
            air_as_any: false,
            threshold: 0.9,
            invalid_nbt: false
        });

        assert_eq!(matches.len(), 1);
    }
}
