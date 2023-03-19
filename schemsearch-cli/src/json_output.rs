use serde::{Deserialize, Serialize};
use schemsearch_lib::SearchBehavior;

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
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub percent: f32,
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