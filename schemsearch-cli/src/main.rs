use std::fmt::format;
use std::fs::File;
use std::io::{BufWriter, Stdout, StdoutLock, Write};
use clap::{command, Arg, ArgAction, ColorChoice, value_parser, Command};
use schemsearch_files::Schematic;
use schemsearch_lib::pattern_mapper::match_palette;
use std::path::Path;
use clap::ArgAction::Help;
use clap::error::ErrorKind;
use schemsearch_lib::{search, SearchBehavior};

fn main() {
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
                .required(true)
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
                .help("Ignores block entities when searching")
                .short('b')
                .long("ignore-block-entities")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-entities")
                .help("Ignores entities when searching")
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
                .value_parser(["std_csv", "file_csv", "std"]),
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

    let schematics = matches.get_many::<String>("schematic").expect("Couldn't get schematics");

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

    let mut stdout = std::io::stdout();
    let mut lock = stdout.lock();

    let mut file: Option<File> = None;
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

    for schem_path in schematics {
        let path = Path::new(schem_path);
        if path.is_dir() {
            match path.read_dir() {
                Ok(x) => {
                    for path in x {
                        match path {
                            Ok(x) => {
                                if x.path().extension().unwrap_or_default() == "schem" {
                                    search_schempath(&mut cmd, search_behavior, &pattern, &mut output_std, &mut output_std_csv, &mut output_file_csv, &mut output_file, &mut lock, &mut file_out, &x.path());
                                }
                            }
                            Err(e) => cmd.error(ErrorKind::Io, format!("Error while reading dir: {}", e.to_string())).exit()
                        }
                    }
                }
                Err(e) => cmd.error(ErrorKind::Io, "Expected to be a dir").exit()
            }
        } else {
            search_schempath(&mut cmd, search_behavior, &pattern, &mut output_std, &mut output_std_csv, &mut output_file_csv, &mut output_file, &mut lock, &mut file_out, path)
        }
    }
}

fn search_schempath(cmd: &mut Command, search_behavior: SearchBehavior, pattern: &Schematic, output_std: &mut bool, output_std_csv: &mut bool, output_file_csv: &mut bool, output_file: &mut bool, stdout: &mut StdoutLock, file_out: &mut Option<BufWriter<File>>, schem_path: &Path) {
    let schematic = match Schematic::load(schem_path) {
        Ok(x) => x,
        Err(e) => {
            cmd.error(ErrorKind::Io, format!("Error while loading Schematic ({}): {}", schem_path.file_name().unwrap().to_str().unwrap(), e.to_string())).exit();
        }
    };

    if *output_std {
        writeln!(stdout, "Searching in schematic: {}", schem_path.file_name().unwrap().to_str().unwrap()).unwrap();
    }

    let matches = search(&schematic, &pattern, search_behavior);

    for x in matches {
        if *output_std {
            writeln!(stdout, "Found match at x: {}, y: {}, z: {}", x.0, x.1, x.2).unwrap();
        }
        if *output_std_csv {
            writeln!(stdout, "{},{},{},{}", schem_path.file_name().unwrap().to_str().unwrap(), x.0, x.1, x.2).unwrap();
        }
        if *output_file {
            writeln!(file_out.as_mut().unwrap(), "Found match at x: {}, y: {}, z: {}", x.0, x.1, x.2).unwrap();
        }
        if *output_file_csv {
            writeln!(file_out.as_mut().unwrap(), "{},{},{},{}", schem_path.file_name().unwrap().to_str().unwrap(), x.0, x.1, x.2).unwrap();
        }
    }
}
