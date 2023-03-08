mod types;

use std::fs::File;
use std::io::{BufWriter, StdoutLock, Write};
use clap::{command, Arg, ArgAction, Command};
use schemsearch_files::Schematic;
use std::path::Path;
use clap::error::ErrorKind;
#[cfg(feature = "sql")]
use futures::executor::block_on;
use schemsearch_lib::{search, SearchBehavior};
#[cfg(feature = "sql")]
use schemsearch_sql::load_all_schematics;
use crate::types::{PathSchematicSupplier, SchematicSupplierType};
#[cfg(feature = "sql")]
use crate::types::SqlSchematicSupplier;

fn main() {
    #[allow(unused_mut)]
    let mut cmd = command!("schemsearch")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("schematic")
                .help("The schematics to search in")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("ignore-data")
                .help("Ignores block data when searching")
                .short('d')
                .long("ignore-data")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-block-entities")
                .help("Ignores block entities when searching [Not Implemented]")
                .short('b')
                .long("ignore-block-entities")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-entities")
                .help("Ignores entities when searching [Not Implemented]")
                .short('e')
                .long("ignore-entities")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-air")
                .help("Ignores air when searching")
                .short('a')
                .long("ignore-air")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("air-as-any")
                .help("Treats air as any block when searching")
                .short('A')
                .long("air-as-any")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .help("The output format")
                .short('o')
                .long("output")
                .action(ArgAction::Append)
                .default_value("std")
                .value_parser(["std_csv", "file_csv", "std", "file"]),
        )
        .arg(
            Arg::new("output-file")
                .help("The output file")
                .short('O')
                .long("output-file")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("threshold")
                .help("The threshold for the search")
                .short('t')
                .long("threshold")
                .action(ArgAction::Set)
                .default_value("0.9")
                .value_parser(|s: &str| s.parse::<f64>().map_err(|e| e.to_string())),
        )
        .about("Searches for a pattern in a schematic")
        .bin_name("schemsearch");

    #[cfg(feature = "sql")]
    let mut cmd = cmd
        .arg(
                Arg::new("sql")
                .help("The sql file to write to")
                .short('s')
                .long("sql")
                .action(ArgAction::SetTrue),
        );

    let matches = cmd.get_matches_mut();

    if matches.contains_id("help") {
        return;
    }

    let search_behavior = SearchBehavior {
        ignore_block_data: matches.get_flag("ignore-data"),
        ignore_block_entities: matches.get_flag("ignore-block-entities"),
        ignore_air: matches.get_flag("ignore-air"),
        air_as_any: matches.get_flag("air-as-any"),
        ignore_entities: matches.get_flag("ignore-entities"),
        threshold: *matches.get_one::<f64>("threshold").expect("Couldn't get threshold"),
    };

    let pattern = match Schematic::load(Path::new(matches.get_one::<String>("pattern").unwrap())) {
        Ok(x) => x,
        Err(e) => {
            cmd.error(ErrorKind::Io, format!("Error while loading Pattern: {}", e.to_string())).exit();
        }
    };

    let mut schematics: Vec<SchematicSupplierType> = match matches.get_many::<String>("schematic") {
        None => vec![],
        Some(x) => x.map(|x| SchematicSupplierType::PATH(Box::new(PathSchematicSupplier{path: Path::new(x) }))).collect()
    };

    #[cfg(feature = "sql")]
    if matches.get_flag("sql") {
        for schem in block_on(load_all_schematics()) {
            schematics.push(SchematicSupplierType::SQL(SqlSchematicSupplier{
                node: schem
            }))
        };
    } else if schematics.is_empty() {
        cmd.error(ErrorKind::MissingRequiredArgument, "No schematics specified").exit();
    }

    let mut output_std = false;
    let mut output_std_csv = false;
    let mut output_file_csv = false;
    let mut output_file = false;

    for x in matches.get_many::<String>("output").expect("Couldn't get output") {
        match x.as_str() {
            "std" => output_std = true,
            "std_csv" => output_std_csv = true,
            "file_csv" => output_file_csv = true,
            "file" => output_file = true,
            _ => {}
        }
    };

    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let file: Option<File>;
    let mut file_out: Option<BufWriter<File>> = None;

    if output_file || output_file_csv {
        let output_file_path = match matches.get_one::<String>("output-file") {
            None => {
                cmd.error(ErrorKind::MissingRequiredArgument, "No output file specified").exit();
            }
            Some(x) => x
        };

        file = match std::fs::File::create(output_file_path) {
            Ok(x) => Some(x),
            Err(e) => {
                cmd.error(ErrorKind::Io, format!("Error while creating output file: {}", e.to_string())).exit();
            }
        };
        file_out = Some(BufWriter::new(file.unwrap()));
    }

    for schem in schematics {
        match schem {
            SchematicSupplierType::PATH(schem) => {
                let path = schem.path;
                if path.is_dir() {
                    match path.read_dir() {
                        Ok(x) => {
                            for path in x {
                                match path {
                                    Ok(x) => {
                                        if x.path().extension().unwrap_or_default() == "schem" {
                                            let schematic = load_schem(&mut cmd, &x.path());
                                            search_schempath(search_behavior, &pattern, &mut output_std, &mut output_std_csv, &mut output_file_csv, &mut output_file, &mut lock, &mut file_out, schematic, x.path().file_name().unwrap().to_str().unwrap().to_string());
                                        }
                                    }
                                    Err(e) => cmd.error(ErrorKind::Io, format!("Error while reading dir: {}", e.to_string())).exit()
                                }
                            }
                        }
                        Err(e) => cmd.error(ErrorKind::Io, format!("Expected to be a dir: {}", e.to_string())).exit()
                    }
                } else {
                    let schematic = load_schem(&mut cmd, &path);
                    search_schempath(search_behavior, &pattern, &mut output_std, &mut output_std_csv, &mut output_file_csv, &mut output_file, &mut lock, &mut file_out, schematic, schem.get_name());
                }
            }
            #[cfg(feature = "sql")]
            SchematicSupplierType::SQL(schem) => {
                search_schempath(search_behavior, &pattern, &mut output_std, &mut output_std_csv, &mut output_file_csv, &mut output_file, &mut lock, &mut file_out, schem.get_schematic().unwrap(), schem.get_name());
            }
        }
    }
}

fn load_schem(cmd: &mut Command, schem_path: &Path) -> Schematic {
    match Schematic::load(schem_path) {
        Ok(x) => x,
        Err(e) => {
            cmd.error(ErrorKind::Io, format!("Error while loading Schematic ({}): {}", schem_path.file_name().unwrap().to_str().unwrap(), e.to_string())).exit();
        }
    }
}

fn search_schempath(search_behavior: SearchBehavior, pattern: &Schematic, output_std: &mut bool, output_std_csv: &mut bool, output_file_csv: &mut bool, output_file: &mut bool, stdout: &mut StdoutLock, file_out: &mut Option<BufWriter<File>>, schematic: Schematic, schem_name: String) {
    if *output_std {
        writeln!(stdout, "Searching in schematic: {}", schem_name).unwrap();
    }
    if *output_file {
        writeln!(file_out.as_mut().unwrap(), "Searching in schematic: {}", schem_name).unwrap();
    }

    let matches = search(schematic, pattern, search_behavior);

    for x in matches {
        if *output_std {
            writeln!(stdout, "Found match at x: {}, y: {}, z: {}, % = {}", x.0, x.1, x.2, x.3).unwrap();
        }
        if *output_std_csv {
            writeln!(stdout, "{},{},{},{},{}", schem_name, x.0, x.1, x.2, x.3).unwrap();
        }
        if *output_file {
            writeln!(file_out.as_mut().unwrap(), "Found match at x: {}, y: {}, z: {}, % = {}", x.0, x.1, x.2, x.3).unwrap();
        }
        if *output_file_csv {
            writeln!(file_out.as_mut().unwrap(), "{},{},{},{},{}", schem_name, x.0, x.1, x.2, x.3).unwrap();
        }
    }
}
