use std::borrow::ToOwned;
use std::collections::HashSet;
use std::iter::Iterator;
use lazy_static::lazy_static;
use schemsearch_files::SpongeSchematic;

const NBT_BLOCKS: &str = include_str!("blocks.txt");

lazy_static! {
    static ref NBT_BLOCKS_SET: HashSet<String> = NBT_BLOCKS.lines().map(ToOwned::to_owned).collect();
}

pub fn has_invalid_nbt(schem: SpongeSchematic) -> bool {
    if schem.block_entities.is_empty() && schem.palette.keys().any(|v| NBT_BLOCKS_SET.contains(v)) {
        return true;
    }

    let nbt_blocks = schem.palette.iter().filter(|(k, _)| NBT_BLOCKS_SET.contains(*k)).map(|(_, v)| *v).collect::<HashSet<i32>>();

    for block_entity in schem.block_data.iter() {
        if nbt_blocks.contains(block_entity) {
            return true;
        }
    }

    return false;
}