use nbt::Map;
use crate::normalize_data;
use crate::schematic::Schematic;

pub(crate) fn strip_data(schem: &Schematic) -> Schematic {
    let mut data: Vec<i32> = Vec::new();

    let mut palette: Map<String, i32> = Map::new();
    let mut palette_max: i32 = 0;

    for block in schem.block_data.iter() {
        let block_name = schem.palette.iter().find(|(_, &v)| v == *block).expect("Invalid Schematic").0;
        let block_name = block_name.split('[').next().unwrap().to_string();

        let entry = palette.entry(block_name).or_insert_with(|| {
            let value = palette_max;
            palette_max += 1;
            value
        });
        data.push(*entry);
    }

    Schematic {
        version: schem.version,
        data_version: schem.data_version,
        palette,
        palette_max,
        block_data: data,
        block_entities: schem.block_entities.clone(),
        height: schem.height,
        length: schem.length,
        width: schem.width,
        metadata: schem.metadata.clone(),
        offset: schem.offset.clone(),
        entities: None,
    }
}

fn match_palette_adapt(schem: &Schematic, matching_palette: Map<String, i32>, ignore_data: bool) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();

    for x in schem.block_data.iter() {
        let blockname = schem.palette.iter().find(|(_, &v)| v == *x).expect("Invalid Schematic").0;
        let blockname = if ignore_data { normalize_data(&blockname, ignore_data) } else { blockname.clone() };
        let block_id = matching_palette.get(&blockname).unwrap_or(&-1);
        data.push(*block_id);
    }

    data
}

pub(crate) fn match_palette(
    schem: &Schematic,
    pattern: &Schematic,
    ignore_data: bool,
) -> (Schematic, Schematic) {
    if ignore_data {
        match_palette_internal(&strip_data(schem), &strip_data(pattern), ignore_data)
    } else {
        match_palette_internal(schem, pattern, ignore_data)
    }
}

fn match_palette_internal(
    schem: &Schematic,
    pattern: &Schematic,
    ignore_data: bool,
) -> (Schematic, Schematic) {

    if schem.palette.len() < pattern.palette.len() {
        panic!("Schematic palette is larger than pattern palette");
    }

    let mut matching_palette: Map<String, i32> = Map::new();
    let mut matching_palette_max: i32 = 0;

    for (block_name, block_id) in pattern.palette.iter() {
        let block_name = normalize_data(block_name, true);
        let schem_block_id = pattern.palette.get(&block_name).expect("Pattern block not found in schematic palette");
        matching_palette.insert(block_name, *schem_block_id);
        matching_palette_max += 1;
    }

    let mut data_schem: Vec<i32> = match_palette_adapt(&schem, matching_palette.clone(), true);

    let mut data_pattern: Vec<i32> = match_palette_adapt(&pattern, matching_palette.clone(), true);

    let schem = Schematic {
        version: schem.version.clone(),
        data_version: schem.data_version.clone(),
        palette: matching_palette.clone(),
        palette_max: matching_palette_max.clone(),
        block_data: data_schem,
        block_entities: schem.block_entities.clone(),
        height: schem.height.clone(),
        length: schem.length.clone(),
        width: schem.width.clone(),
        metadata: schem.metadata.clone(),
        offset: schem.offset.clone(),
        entities: None,
    };
    let pattern = Schematic {
        version: pattern.version.clone(),
        data_version: pattern.data_version.clone(),
        palette: matching_palette.clone(),
        palette_max: matching_palette_max.clone(),
        block_data: data_pattern,
        block_entities: pattern.block_entities.clone(),
        height: pattern.height.clone(),
        length: pattern.length.clone(),
        width: pattern.width.clone(),
        metadata: pattern.metadata.clone(),
        offset: pattern.offset.clone(),
        entities: None,
    };

    (schem, pattern)
}