use pattern_mapper::match_palette;
use schemsearch_files::Schematic;

pub mod pattern_mapper;

#[derive(Debug, Clone, Copy)]
pub struct SearchBehavior {
    pub ignore_block_data: bool,
    pub ignore_block_entities: bool,
    pub ignore_entities: bool,
}

pub fn search(
    data: &Vec<u8>,
    pattern: &Vec<u8>,
    search_behavior: SearchBehavior,
) -> Vec<(u16, u16, u16)> {
    let schem: Schematic = parse_schematic(data);
    let pattern_schem: Schematic = parse_schematic(pattern);

    if schem.width < pattern_schem.width || schem.height < pattern_schem.height || schem.length < pattern_schem.length {
        return vec![];
    }

    if pattern_schem.palette.len() > schem.palette.len() {
        return vec![];
    }

    let (schem, pattern_schem) = match_palette(&schem, &pattern_schem, search_behavior.ignore_block_data);

    let mut matches: Vec<(u16, u16, u16)> = Vec::new();

    println!("{:?}", schem);
    println!("{:?}", pattern_schem);

    let pattern_data = pattern_schem.read_blockdata();
    let schem_data = schem.read_blockdata();

    for x in 0..=schem.width - pattern_schem.width {
        for y in 0..=schem.height - pattern_schem.height {
            for z in 0..=schem.length - pattern_schem.length {
                let mut match_found = true;
                'outer: for i in 0..pattern_schem.width {
                    for j in 0..pattern_schem.height {
                        for k in 0..pattern_schem.length {
                            let index = (x + i) + (y + j) * schem.width + (z + k) * schem.width * schem.height;
                            let pattern_index = i + j * pattern_schem.width + k * pattern_schem.width * pattern_schem.height;
                            if schem_data.get(index as usize) != pattern_data.get(pattern_index as usize) {
                                match_found = false;
                                break 'outer;
                            }
                        }
                    }
                }
                if match_found {
                    matches.push((x, y, z));
                }
            }
        }
    }

    /*
    [
     0, -1,  1, 1,  2,
     0, -1,  2, 1,  0,
     2, -1, -1, 2, -1,
     2,  0,  0, 2, -1,
     2,  1,  2, 2,  1
     ]

     */

    return matches;

}

pub fn normalize_data(data: &String, ignore_data: bool) -> String {
    if ignore_data {
        data.split('[').next().unwrap().to_string()
    } else {
        data.clone()
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
        let schematic = Schematic::load(Path::new("../tests/simple.schem"));
        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
        println!("{:?}", schematic);
    }

    #[test]
    fn test_parse_function() {
        let file = std::fs::File::open("../tests/simple.schem").expect("Failed to open file");
        let schematic: Schematic = parse_schematic(&std::io::Read::bytes(file).map(|b| b.unwrap()).collect());
        assert_eq!(schematic.width as usize * schematic.height as usize * schematic.length as usize, schematic.block_data.len());
        assert_eq!(schematic.palette_max, schematic.palette.len() as i32);
        println!("{:?}", schematic);
    }

    #[test]
    fn test_strip_schem() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem"));
        let stripped = strip_data(&schematic);

        assert_eq!(stripped.palette.keys().any(|k| k.contains('[')), false);
        println!("{:?}", stripped);
    }

    #[test]
    fn test_match_palette() {
        let schematic = Schematic::load(Path::new("../tests/simple.schem"));
        let endstone = Schematic::load(Path::new("../tests/endstone.schem"));

        let (matched_schematic, matched_endstone) = match_palette(&schematic, &endstone, true);

        println!("{:?}", matched_schematic);
        println!("{:?}", matched_endstone);
    }

    #[test]
    pub fn test_search() {
        let file = std::fs::File::open("../tests/Random.schem").expect("Failed to open file");
        let schematic = &std::io::Read::bytes(file).map(|b| b.unwrap()).collect();
        println!("{:?}", schematic);
        let file = std::fs::File::open("../tests/Pattern.schem").expect("Failed to open file");
        let pattern = &std::io::Read::bytes(file).map(|b| b.unwrap()).collect();

        let matches = search(schematic, pattern, SearchBehavior {
            ignore_block_data: true,
            ignore_block_entities: true,
            ignore_entities: true,
        });

        println!("{:?}", matches);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], (1, 0, 3));
    }
}
