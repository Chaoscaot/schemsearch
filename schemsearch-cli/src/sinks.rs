use std::fs::File;
use std::io::BufWriter;
use std::str::FromStr;
use std::io::Write;
use std::time::Duration;
use indicatif::HumanDuration;
use schemsearch_lib::{Match, SearchBehavior};
use crate::json_output::{EndEvent, FoundEvent, InitEvent, JsonEvent};

#[derive(Debug, Clone)]
pub enum OutputSink {
    Stdout,
    Stderr,
    File(String),
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Text,
    CSV,
    JSON
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(OutputFormat::Text),
            "csv" => Ok(OutputFormat::CSV),
            "json" => Ok(OutputFormat::JSON),
            _ => Err(format!("'{}' is not a valid output format", s))
        }
    }
}

impl FromStr for OutputSink {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "std" => Ok(OutputSink::Stdout),
            "err" => Ok(OutputSink::Stderr),
            _ => Ok(OutputSink::File(s.to_string()))
        }
    }
}

impl OutputSink {
    pub fn output(&self) -> Box<dyn Write> {
        match self {
            OutputSink::Stdout => Box::new(std::io::stdout().lock()),
            OutputSink::Stderr => Box::new(std::io::stderr().lock()),
            OutputSink::File(path) => Box::new(BufWriter::new(File::create(path).unwrap()))
        }
    }
}

impl OutputFormat {
    pub fn found_match(&self, name: &String, pos: Match) -> String {
        match self {
            OutputFormat::Text => format!("Found match in '{}' at x: {}, y: {}, z: {}, % = {}\n", name, pos.x, pos.y, pos.z, pos.percent),
            OutputFormat::CSV => format!("{},{},{},{},{}\n", name, pos.x, pos.y, pos.z, pos.percent),
            OutputFormat::JSON => format!("{}\n", serde_json::to_string(&JsonEvent::Found(FoundEvent {
                name: name.clone(),
                match_: pos,
            })).unwrap())
        }
    }

    pub fn start(&self, total: u32, search_behavior: &SearchBehavior, start_time: u128) -> String {
        match self {
            OutputFormat::Text => format!("Starting search in {} schematics\n", total),
            OutputFormat::CSV => format!("Name,X,Y,Z,Percent\n"),
            OutputFormat::JSON => format!("{}\n", serde_json::to_string(&JsonEvent::Init(InitEvent {
                total,
                search_behavior: search_behavior.clone(),
                start_time,
            })).unwrap())
        }
    }

    pub fn end(&self, end_time: u128) -> String {
        match self {
            OutputFormat::Text => format!("Search complete in {}\n", HumanDuration(Duration::from_millis(end_time as u64))),
            OutputFormat::CSV => format!("{}\n", end_time),
            OutputFormat::JSON => format!("{}\n", serde_json::to_string(&JsonEvent::End(EndEvent{ end_time })).unwrap())
        }
    }
}