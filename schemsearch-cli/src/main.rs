/*
 * Copyright (C) 2023  Chaoscaot
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod types;
mod json_output;
mod sinks;
mod stderr;

use std::fmt::Debug;
use std::io::Write;
use clap::{command, Arg, ArgAction, ValueHint};
use std::path::PathBuf;
use std::str::FromStr;
use clap::error::ErrorKind;
use schemsearch_lib::{Match, SearchBehavior};
use crate::types::{PathSchematicSupplier, SchematicSupplier, SchematicSupplierType};
#[cfg(feature = "sql")]
use futures::executor::block_on;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
#[cfg(feature = "sql")]
use schemsearch_sql::filter::SchematicFilter;
#[cfg(feature = "sql")]
use schemsearch_sql::load_all_schematics;
#[cfg(feature = "sql")]
use crate::types::SqlSchematicSupplier;
use indicatif::*;
use schemsearch_files::SpongeSchematic;
use crate::sinks::{OutputFormat, OutputSink};
use crate::stderr::MaschineStdErr;
use schemsearch_lib::nbt_search::has_invalid_nbt;
use schemsearch_lib::search::search;

fn main() {
    #[allow(unused_mut)]
        let mut cmd = command!("schemsearch")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .value_hint(ValueHint::FilePath)
                .required_unless_present("invalid-nbt")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("schematic")
                .help("The schematics to search in")
                .value_hint(ValueHint::AnyPath)
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
            Arg::new("invalid-nbt")
                .help("Search for Schematics with Invalid or missing NBT data")
                .short('I')
                .long("invalid-nbt")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .help("The output format and path [Format:Path] available formats: text, json, csv; available paths: std, err, (file path)")
                .short('o')
                .long("output")
                .action(ArgAction::Append)
                .default_value("text:std")
                .value_parser(|s: &str| {
                    let mut split = s.splitn(2, ':');
                    let format = match split.next() {
                        None => return Err("No format specified".to_string()),
                        Some(x) => x
                    };
                    let path = match split.next() {
                        None => return Err("No path specified".to_string()),
                        Some(x) => x
                    };
                    let format = match OutputFormat::from_str(format) {
                        Ok(x) => x,
                        Err(e) => return Err(e.to_string()),
                    };
                    let path = match OutputSink::from_str(path) {
                        Ok(x) => x,
                        Err(e) => return Err(e.to_string()),
                    };

                    Ok((format, path))
                }),
        )
        .arg(
            Arg::new("threshold")
                .help("The threshold for the search")
                .short('t')
                .long("threshold")
                .action(ArgAction::Set)
                .default_value("0.9")
                .value_parser(|s: &str| s.parse::<f32>().map_err(|e| e.to_string())),
        )
        .arg(
            Arg::new("threads")
                .help("The number of threads to use [0 = all Available Threads]")
                .short('T')
                .long("threads")
                .action(ArgAction::Set)
                .default_value("0")
                .value_parser(|s: &str| s.parse::<usize>().map_err(|e| e.to_string())),
        )
        .arg(
            Arg::new("machine")
                .help("Output for machines")
                .short('m')
                .long("machine")
                .action(ArgAction::Set)
                .default_value("0")
                .value_parser(|s: &str| s.parse::<u16>().map_err(|e| e.to_string()))
        )
        .arg(
            Arg::new("limit")
                .help("The maximum number of matches to return [0 = Unlimited]")
                .short('l')
                .long("limit")
                .action(ArgAction::Set)
                .default_value("50")
                .value_parser(|s: &str| s.parse::<usize>().map_err(|e| e.to_string())),
        )
        .about("Searches for a pattern in a schematic")
        .bin_name("schemsearch");

    #[cfg(feature = "sql")]
        let mut cmd = cmd
        .arg(
            Arg::new("sql")
                .help("Use the SteamWar SQL Database")
                .short('s')
                .long("sql")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sql-filter-user")
                .help("Filter the schematics by the owners userid")
                .short('u')
                .long("sql-filter-user")
                .action(ArgAction::Append)
                .value_parser(|s: &str| s.parse::<u32>().map_err(|e| e.to_string()))
                .requires("sql"),
        )
        .arg(
            Arg::new("sql-filter-name")
                .help("Filter the schematics by the schematic name")
                .short('n')
                .long("sql-filter-name")
                .action(ArgAction::Append)
                .requires("sql"),
        );

    let matches = cmd.get_matches_mut();

    if matches.contains_id("help") {
        return;
    }

    let start = std::time::Instant::now();

    let search_behavior = SearchBehavior {
        ignore_block_data: matches.get_flag("ignore-data"),
        ignore_block_entities: matches.get_flag("ignore-block-entities"),
        ignore_air: matches.get_flag("ignore-air"),
        air_as_any: matches.get_flag("air-as-any"),
        ignore_entities: matches.get_flag("ignore-entities"),
        threshold: *matches.get_one::<f32>("threshold").expect("Couldn't get threshold"),
        invalid_nbt: matches.get_flag("invalid-nbt"),
    };

    let pattern = match matches.get_one::<String>("pattern") {
        Some(p) => match SpongeSchematic::load(&PathBuf::from(p)) {
            Ok(x) => Some(x),
            Err(e) => {
                cmd.error(ErrorKind::Io, format!("Error while loading Pattern: {}", e.to_string())).exit();
            }
        },
        None => None,
    };

    let mut schematics: Vec<SchematicSupplierType> = Vec::new();
    match matches.get_many::<String>("schematic") {
        None => {}
        Some(x) => {
            let paths = x.map(|x| PathBuf::from(x));
            for path in paths {
                if path.is_dir() {
                    path.read_dir()
                        .expect("Couldn't read directory")
                        .filter_map(|x| x.ok())
                        .filter(|x| x.path().is_file())
                        .filter(|x| x.path().extension().unwrap().to_str().unwrap() == "schem")
                        .for_each(|x| {
                            schematics.push(SchematicSupplierType::PATH(PathSchematicSupplier {
                                path: x.path(),
                            }))
                        });
                } else if path.extension().unwrap().to_str().unwrap() == "schem" {
                    schematics.push(SchematicSupplierType::PATH(PathSchematicSupplier { path }));
                }
            }
        }
    };

    #[cfg(feature = "sql")]
    if matches.get_flag("sql") {
        let mut filter = SchematicFilter::default();
        if let Some(x) = matches.get_many::<u32>("sql-filter-user") {
            filter = filter.user_id(x.collect());
        }
        if let Some(x) = matches.get_many::<String>("sql-filter-name") {
            filter = filter.name(x.collect());
        }
        for schem in block_on(load_all_schematics(filter)) {
            schematics.push(SchematicSupplierType::SQL(SqlSchematicSupplier {
                node: schem
            }))
        };
    }

    if schematics.is_empty() {
        cmd.error(ErrorKind::MissingRequiredArgument, "No schematics specified").exit();
    }

    let output: Vec<&(OutputFormat, OutputSink)> = matches.get_many::<(OutputFormat, OutputSink)>("output").expect("Error").collect();
    let mut output: Vec<(OutputFormat, Box<dyn Write>)> = output.into_iter().map(|x| (x.0.clone(), x.1.output())).collect();

    for x in &mut output {
        write!(x.1, "{}", x.0.start(schematics.len() as u32, &search_behavior, start.elapsed().as_millis())).unwrap();
    }

    ThreadPoolBuilder::new().num_threads(*matches.get_one::<usize>("threads").expect("Could not get threads")).build_global().unwrap();

    let bar = ProgressBar::new(schematics.len() as u64); // "maschine"
    bar.set_style(ProgressStyle::with_template("[{elapsed}, ETA: {eta}] {wide_bar} {pos}/{len} {per_sec}").unwrap());
    let term_size = *matches.get_one::<u16>("machine").expect("Could not get machine");
    if term_size != 0 {
        bar.set_draw_target(ProgressDrawTarget::term_like(Box::new(MaschineStdErr { size: term_size })))
    }

    let max_matching = *matches.get_one::<usize>("limit").expect("Could not get max-matching");

    let matches: Vec<SearchResult> = schematics.par_iter().progress_with(bar).map(|schem| {
        match schem {
            SchematicSupplierType::PATH(schem) => {
                let schematic = match load_schem(&schem.path) {
                    Some(x) => x,
                    None => return SearchResult {
                        name: schem.get_name(),
                        matches: Vec::default(),
                    }
                };
                search_in_schem(schematic, pattern.as_ref(), search_behavior, schem)
            }
            #[cfg(feature = "sql")]
            SchematicSupplierType::SQL(schem) => {
                match schem.get_schematic() {
                    Ok(schematic) => search_in_schem(schematic, pattern.as_ref(), search_behavior, schem),
                    Err(e) => {
                        eprintln!("Error while loading schematic ({}): {}", schem.get_name(), e.to_string());
                        SearchResult {
                            name: schem.get_name(),
                            matches: Vec::default(),
                        }
                    }
                }
            }
        }
    }).collect();

    let mut matches_count = 0;

    'outer: for matching in matches {
        let schem_name = matching.name;
        let matching = matching.matches;
        for x in matching {
            for out in &mut output {
                write!(out.1, "{}", out.0.found_match(&schem_name, x)).unwrap();
            }
            matches_count += 1;
            if max_matching != 0 && matches_count >= max_matching {
                break 'outer;
            }
        }
    }

    let end = std::time::Instant::now();
    for x in &mut output {
        write!(x.1, "{}", x.0.end(end.duration_since(start).as_millis())).unwrap();
        x.1.flush().unwrap();
    }
}

fn search_in_schem(schematic: SpongeSchematic, pattern: Option<&SpongeSchematic>, search_behavior: SearchBehavior, schem: &impl SchematicSupplier) -> SearchResult {
    if search_behavior.invalid_nbt {
        if has_invalid_nbt(schematic) {
            SearchResult {
                name: schem.get_name(),
                matches: vec![Match {
                    x: 0,
                    y: 0,
                    z: 0,
                    percent: 1.0,
                }],
            }
        } else {
            SearchResult {
                name: schem.get_name(),
                matches: vec![],
            }
        }
    } else {
        SearchResult {
            name: schem.get_name(),
            matches: search(schematic, pattern.unwrap(), search_behavior),
        }
    }
}

fn load_schem(schem_path: &PathBuf) -> Option<SpongeSchematic> {
    match SpongeSchematic::load(schem_path) {
        Ok(x) => Some(x),
        Err(e) => {
            println!("Error while loading schematic ({}): {}", schem_path.to_str().unwrap(), e.to_string());
            None
        }
    }
}

#[derive(Debug, Clone)]
struct SearchResult {
    name: String,
    matches: Vec<Match>,
}

