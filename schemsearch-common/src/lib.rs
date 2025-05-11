use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SearchBehavior {
    pub ignore_block_data: bool,
    pub ignore_block_entities: bool,
    pub ignore_air: bool,
    pub air_as_any: bool,
    pub ignore_entities: bool,
    pub threshold: f32,
    pub invalid_nbt: bool,
    pub opencl: bool,
}

impl Default for SearchBehavior {
    fn default() -> Self {
        SearchBehavior {
            ignore_block_data: false,
            ignore_block_entities: false,
            ignore_air: false,
            air_as_any: false,
            ignore_entities: false,
            threshold: 0.9,
            invalid_nbt: false,
            opencl: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Match {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub percent: f32,
}

#[macro_export]
macro_rules! time {
    ($name:ident, $body:block) => {
        {
            #[cfg(debug_assertions)]
            {
                let start = std::time::Instant::now();
                let result = $body;
                let duration = start.elapsed();
                println!("{} took {:?}", stringify!($name), duration);
                result
            }
            #[cfg(not(debug_assertions))]
            {
                $body
            }
        }
    };
}
