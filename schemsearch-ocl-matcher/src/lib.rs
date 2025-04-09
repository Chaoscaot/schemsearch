use std::sync::OnceLock;
use ocl::{Buffer, MemFlags, ProQue, Platform};
use ocl::SpatialDims::Three;
use schemsearch_common::{Match, SearchBehavior};
use math::round::ceil;

const KERNEL: &str = include_str!("kernel.cl");

static PRO_QUEU_CELL: OnceLock<ProQue> = OnceLock::new();

pub fn ocl_available() -> bool {
    !Platform::list().is_empty()
}

pub fn ocl_search(
    schem: &[i32],
    schem_size: [usize; 3],
    pattern: &[i32],
    pattern_size: [usize; 3],
    air_id: i32,
    search_behavior: SearchBehavior,
) -> Result<Vec<Match>, String> {
    search_ocl(schem, schem_size, pattern, pattern_size, air_id, search_behavior).map_err(|e| e.to_string())
}

fn search_ocl(
    schem: &[i32],
    schem_size: [usize; 3],
    pattern: &[i32],
    pattern_size: [usize; 3],
    air_id: i32,
    search_behavior: SearchBehavior,
) -> ocl::Result<Vec<Match>> {
    let pattern_width = pattern_size[0];
    let pattern_height = pattern_size[1];
    let pattern_length = pattern_size[2];

    let schem_width = schem_size[0];
    let schem_height = schem_size[1];
    let schem_length = schem_size[2];
    
    let pattern_blocks = (pattern_width * pattern_height * pattern_length) as f32;
    
    let skip_amount = ceil((pattern_blocks * (1.0 - search_behavior.threshold)) as f64, 0) as i32;

    let cell = &PRO_QUEU_CELL;
    let mut pro_que = cell.get_or_init(|| {
        ProQue::builder()
            .src(KERNEL)
            .build().unwrap()
    }).clone();


    pro_que.set_dims(Three(schem_width, schem_height, schem_length));

    let buffer = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write())
        .fill_val(-1)
        .len(schem.len())
        .build()?;

    let schem_buffer = create_schem_buffer(schem, &pro_que)?;

    let pattern_buffer = create_schem_buffer(pattern, &pro_que)?;

    let kernel = pro_que.kernel_builder("add")
        .arg(&buffer)
        .arg(&schem_buffer)
        .arg(&pattern_buffer)
        .arg(pattern_width as i32)
        .arg(pattern_height as i32)
        .arg(pattern_length as i32)
        .arg(air_id) // air_id
        .arg(search_behavior.ignore_air as u32) // ignore_air
        .arg(search_behavior.air_as_any as u32) // air_as_any
        .arg(skip_amount)
        .build()?;

    unsafe { kernel.enq()?; }

    let mut vec = vec![0i32; buffer.len()];
    buffer.read(&mut vec).enq()?;
    
    Ok(vec.into_iter().enumerate().filter(|(_, v)| *v < skip_amount && *v != -1).map(|(i, v)| {
        Match {
            x: (i % schem_width) as u16,
            y: ((i / (schem_width * schem_length)) % schem_height) as u16,
            z: ((i / schem_width) % schem_length) as u16,

            percent: (pattern_blocks - v as f32) / pattern_blocks,
        }
    }).collect())
}

fn create_schem_buffer(pattern: &[i32], pro_que: &ProQue) -> ocl::Result<Buffer<i32>> {
    Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only())
        .len(pattern.len())
        .copy_host_slice(pattern)
        .build()
}