use std::path::Path;
use schemsearch_files::Schematic;
use schemsearch_lib::pattern_mapper::match_palette;

fn main() {
    let schematic = Schematic::load(Path::new("tests/simple.schem"));
    let endstone = Schematic::load(Path::new("tests/endstone.schem"));

    let (matched_schematic, matched_endstone) = match_palette(&schematic, &endstone, true);

    println!("{:?}", matched_schematic);
    println!("{:?}", matched_endstone);
}
