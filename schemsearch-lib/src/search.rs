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

    let skip_amount = ceil((pattern_blocks * (1.0 - search_behavior.threshold)) as f64, 0) as i32;

    for y in 0..=schem_height - pattern_height {
        for z in 0..=schem_length - pattern_length {
            for x in 0..=schem_width - pattern_width {
                let mut not_matching = 0;
                'outer:
                for j in 0..pattern_height {
                    for k in 0..pattern_length {
                        'inner:
                        for i in 0..pattern_width {
                            let index = (x + i) + schem_width * ((z + k) + (y + j) * schem_length);
                            let pattern_index = i + pattern_width * (k + j * pattern_length);
                            let data = unsafe { *schem_data.add(index) };
                            let pattern_data = unsafe { *pattern_data.add(pattern_index) };
                            if (search_behavior.ignore_air && data != *air_id) || (search_behavior.air_as_any && pattern_data != *air_id) {
                                continue 'inner;
                            }
                            if data != pattern_data {
                                not_matching += 1;
                                if not_matching >= skip_amount {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                if not_matching < skip_amount {
                    matches.push(Match {
                        x: x as u16,
                        y: y as u16,
                        z: z as u16,
                        percent: (i_pattern_blocks - not_matching) as f32 / pattern_blocks,
                    });
                }
            }
        }
    }

    return matches;
}