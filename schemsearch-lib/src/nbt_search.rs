use std::borrow::ToOwned;
use std::collections::HashSet;
use std::iter::Iterator;
use lazy_static::lazy_static;
use schemsearch_files::SpongeSchematic;

const NBT_BLOCKS: &str = include_str!("blocks.txt");

lazy_static! {
    static ref NBT_BLOCKS_SET: HashSet<String> = {
        NBT_BLOCKS.lines().map(|x| format!("minecraft:{}", x)).collect()
    };
}

pub fn has_invalid_nbt(schem: SpongeSchematic) -> bool {
    if schem.block_entities.is_empty() && schem.palette.keys().any(|v| NBT_BLOCKS_SET.contains(v)) {
        return true;
    }

    let nbt_blocks = schem.palette.iter().filter(|(k, _)| NBT_BLOCKS_SET.contains(k.to_owned())).map(|(_, v)| *v).collect::<HashSet<i32>>();

    for (i, block_entity) in schem.block_data.iter().enumerate() {
        if nbt_blocks.contains(&*block_entity) {
            // i = x + z * Width + y * Width * Length
            let x = i % schem.width as usize;
            let z = (i / schem.width as usize) % schem.length as usize;
            let y = i / (schem.width as usize * schem.length as usize);
            if schem.block_entities.iter().any(|e| !e.pos.eq(&[x as i32, y as i32, z as i32])) {
                return true;
            }
        }
    }

    return false;
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use nbt::CompoundTag;
    use schemsearch_files::{BlockEntity, SpongeSchematic};
    use super::*;

    #[test]
    fn test_has_invalid_nbt() {
        let schem = SpongeSchematic {
            data_version: 1,
            metadata: CompoundTag::new(),
            width: 0,
            height: 0,
            length: 0,
            offset: [0, 0, 0],
            palette_max: 1,
            palette: vec![("minecraft:chest".to_owned(), 1)].into_iter().collect(),
            block_data: vec![1],
            block_entities: vec![],
            entities: None,
        };

        assert_eq!(has_invalid_nbt(schem), true);
    }

    #[test]
    fn test_has_invalid_nbt_2() {
        let schem = SpongeSchematic {
            data_version: 1,
            metadata: CompoundTag::new(),
            width: 1,
            height: 1,
            length: 1,
            offset: [0, 0, 0],
            palette_max: 1,
            palette: vec![("minecraft:chest".to_owned(), 1)].into_iter().collect(),
            block_data: vec![1],
            block_entities: vec![
                BlockEntity {
                    id: "minecraft:chest".to_owned(),
                    pos: [0, 0, 0],
                }
            ],
            entities: None,
        };

        assert_eq!(has_invalid_nbt(schem), false);
    }

    #[test]
    fn test_has_invalid_nbt_3() {
        let schem = SpongeSchematic {
            data_version: 1,
            metadata: CompoundTag::new(),
            width: 2,
            height: 1,
            length: 1,
            offset: [0, 0, 0],
            palette_max: 1,
            palette: vec![("minecraft:chest".to_owned(), 1), ("minecraft:stone".to_owned(), 2)].into_iter().collect(),
            block_data: vec![1, 2],
            block_entities: vec![
                BlockEntity {
                    id: "minecraft:chest".to_owned(),
                    pos: [1, 0, 0],
                }
            ],
            entities: None,
        };

        assert_eq!(has_invalid_nbt(schem), true);
    }
}