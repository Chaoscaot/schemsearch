use nbt::Map;
use schemsearch_files::Schematic;
use crate::normalize_data;

fn create_reverse_palette(schem: &Schematic) -> Vec<String> {
    let mut reverse_palette = Vec::with_capacity(schem.palette_max as usize);
    (0..schem.palette_max).for_each(|_| reverse_palette.push(String::new()));
    for (key, value) in schem.palette.iter() {
        reverse_palette[*value as usize] = key.clone();
    }
    reverse_palette
}

pub fn strip_data(schem: &Schematic) -> Schematic {
    let mut data: Vec<i32> = Vec::new();

    let mut palette: Map<String, i32> = Map::new();
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

pub fn match_palette_adapt(schem: &Schematic, matching_palette: &Map<String, i32>, ignore_data: bool) -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();
    let reverse_palette = create_reverse_palette(schem);

    for x in &schem.block_data {
        let blockname = &reverse_palette[*x as usize];
        let blockname = if ignore_data { normalize_data(&blockname, ignore_data) } else { blockname.clone() };
        let block_id = match matching_palette.get(&blockname) {
            None => -1,
            Some(x) => *x
        };
        data.push(block_id);
    }

    data
}

pub fn match_palette(
    schem: &Schematic,
    pattern: &Schematic,
    ignore_data: bool,
) -> Schematic {
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
) -> Schematic {
    let data_pattern: Vec<i32> = match_palette_adapt(&pattern, &schem.palette, ignore_data);

    Schematic {
        version: pattern.version.clone(),
        data_version: pattern.data_version.clone(),
        palette: schem.palette.clone(),
        palette_max: schem.palette_max,
        block_data: data_pattern,
        block_entities: pattern.block_entities.clone(),
        height: pattern.height.clone(),
        length: pattern.length.clone(),
        width: pattern.width.clone(),
        metadata: pattern.metadata.clone(),
        offset: pattern.offset.clone(),
        entities: None,
    }
}