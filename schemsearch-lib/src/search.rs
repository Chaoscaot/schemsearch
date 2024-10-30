use math::round::ceil;
use schemsearch_files::SpongeSchematic;
use crate::{Match, SearchBehavior};
use crate::pattern_mapper::{match_palette, match_palette_adapt};

pub fn search(
    schem: SpongeSchematic,
    pattern_schem: &SpongeSchematic,
    search_behavior: SearchBehavior,
) -> Vec<Match> {
    if schem.width < pattern_schem.width || schem.height < pattern_schem.height || schem.length < pattern_schem.length {
        return Vec::new();
    }

    if pattern_schem.palette.len() > schem.palette.len() {
        return Vec::new();
    }

    let pattern_schem = match_palette(&schem, &pattern_schem, search_behavior.ignore_block_data);

    let mut matches: Vec<Match> = Vec::with_capacity(4);

    let pattern_data = pattern_schem.block_data.as_ptr();

    let schem_data = if search_behavior.ignore_block_data {
        match_palette_adapt(&schem, &pattern_schem.palette, search_behavior.ignore_block_data)
    } else {
        schem.block_data
    };

    let schem_data = schem_data.as_ptr();

    let air_id = if search_behavior.ignore_air || search_behavior.air_as_any { pattern_schem.palette.get("minecraft:air").unwrap_or(&-1) } else { &-1};

    let pattern_blocks = pattern_schem.block_data.len() as f32;
    let i_pattern_blocks = pattern_blocks as i32;

    let pattern_width = pattern_schem.width as usize;
    let pattern_height = pattern_schem.height as usize;
    let pattern_length = pattern_schem.length as usize;

    let schem_width = schem.width as usize;
    let schem_height = schem.height as usize;
    let schem_length = schem.length as usize;

    let mut pattern_vec_length = 0;

    for i in 0..pattern_schem.block_data.len() {
        pattern_vec_length += unsafe { *pattern_data.add(i) * *pattern_data.add(i) };
    }

    let skip_amount = ceil((pattern_blocks * (1.0 - search_behavior.threshold)) as f64, 0) as i32;

    for y in 0..=schem_height - pattern_height {
        for z in 0..=schem_length - pattern_length {
            for x in 0..=schem_width - pattern_width {
                let mut dot_p: i32 = 0;
                let mut schem_vec_length = 0;

                for i in 0..pattern_schem.block_data.len() {
                    let k = i % pattern_width;
                    let j = (i / pattern_width) % pattern_length;
                    let m = i / (pattern_width * pattern_length);

                    let schem_index = (x + k) + schem_width * ((z + j) + (y + m) * schem_length);
                    dot_p += unsafe { *pattern_data.add(i) * *schem_data.add(schem_index) };

                    schem_vec_length += unsafe { *schem_data.add(schem_index) * *schem_data.add(schem_index) };
                }

                let sim = dot_p as f32 / ((pattern_vec_length as f32).sqrt() * (schem_vec_length as f32).sqrt());

                if sim > search_behavior.threshold {
                    matches.push(Match {
                        x: x as u16,
                        y: y as u16,
                        z: z as u16,
                        percent: sim,
                    });
                }
            }
        }
    }

    return matches;
}