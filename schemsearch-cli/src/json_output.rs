use serde::{Deserialize, Serialize};
use schemsearch_lib::{Match, SearchBehavior};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
pub enum JsonEvent {
    Found(FoundEvent),
    Init(InitEvent),
    End(EndEvent),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FoundEvent {
    pub name: String,
    #[serde(flatten, rename = "match")]
    pub match_: Match,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitEvent {
    pub total: u32,
    pub search_behavior: SearchBehavior,
    pub start_time: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndEvent {
    pub end_time: u128,
}