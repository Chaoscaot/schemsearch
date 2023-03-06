pub mod pattern_mapper;

use pattern_mapper::match_palette;
use schemsearch_files::Schematic;

#[derive(Debug, Clone, Copy)]
pub struct SearchBehavior {
    pub ignore_block_data: bool,
    pub ignore_block_entities: bool,
    pub ignore_air: bool,
    pub air_as_any: bool,
    pub ignore_entities: bool,
    pub threshold: f64,
}

pub fn search(
    schem: &Schematic,
    pattern_schem: &Schematic,
    search_behavior: SearchBehavior,
) -> Vec<(u16, u16, u16, f64)> {
    if schem.width < pattern_schem.width || schem.height < pattern_schem.height || schem.length < pattern_schem.length {
        return vec![];
    }

    if pattern_schem.palette.len() > schem.palette.len() {
        return vec![];
    }

    let pattern_schem = match match_palette(&schem, &pattern_schem, search_behavior.ignore_block_data) {
        Some(x) => x,
        None => return vec![],
    };

    let mut matches: Vec<(u16, u16, u16, f64)> = Vec::new();

    let pattern_data = pattern_schem.block_data;
    let schem_data = &schem.block_data;
    let air_id = if search_behavior.ignore_air || search_behavior.air_as_any { pattern_schem.palette.get("minecraft:air").unwrap_or(&-1) } else { &-1};

    let pattern_blocks = (pattern_schem.width * pattern_schem.height * pattern_schem.length) as f64;

    for x in 0..=schem.width as usize - pattern_schem.width as usize {
        for y in 0..=schem.height as usize - pattern_schem.height as usize {
            for z in 0..=schem.length as usize - pattern_schem.length as usize {
                let mut matching = 0;
                for i in 0..pattern_schem.width as usize {
                    for j in 0..pattern_schem.height as usize {
                        for k in 0..pattern_schem.length as usize {
                            let index = (x + i) + (y + j) * (schem.width as usize) + (z + k) * (schem.width as usize) * (schem.height as usize);
                            let pattern_index = i + j * pattern_schem.width as usize + k * pattern_schem.width as usize * pattern_schem.height as usize;
                            let data = schem_data.get(index as usize).expect("Index out of bounds");
                            let pattern_data = pattern_data.get(pattern_index as usize).expect("Index out of bounds");
                            if data == pattern_data || (search_behavior.ignore_air && *data == *air_id) || (search_behavior.air_as_any && *pattern_data == *air_id) {
                                matching += 1;
                            }
                        }
                    }
                }
                let matching_percent = matching as f64 / pattern_blocks;
                if matching_percent > search_behavior.threshold {
                    matches.push((x as u16, y as u16, z as u16, matching_percent));
                }
            }
        }
    }

    return matches;
}

pub fn normalize_data(data: &String, ignore_data: bool) -> String {
    if ignore_data {
        data.split('[').next().unwrap().to_string()
    } else {
        data.to_string()
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
mod tests {
    use std::path::Path;
    use schemsearch_files::Schematic;
    use crate::pattern_mapper::strip_data;
    use super::*;

    #[test]
    fn read_schematic() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem")).unwrap();
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
        let schematic = Schematic::load(Path::new("../tests/simple.schem")).unwrap();
        let stripped = strip_data(&schematic);

        assert_eq!(stripped.palette.keys().any(|k| k.contains('[')), false);
    }

    #[test]
    fn test_match_palette() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(Path::new("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, true);
    }

    #[test]
    fn test_match_palette_ignore_data() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(Path::new("../tests/endstone.schem")).unwrap();

        let _ = match_palette(&schematic, &endstone, false);
    }

    #[test]
    pub fn test_big_search() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem")).unwrap();
        let endstone = Schematic::load(Path::new("../tests/endstone.schem")).unwrap();

        let _ = search(&schematic, &endstone, SearchBehavior {
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
        let schematic = Schematic::load(Path::new("../tests/Random.schem")).unwrap();
        let pattern = Schematic::load(Path::new("../tests/Pattern.schem")).unwrap();

        let matches = search(&schematic, &pattern, SearchBehavior {
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
}
