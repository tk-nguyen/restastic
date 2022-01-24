use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub id: String,
    pub chunker_polynomial: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
    pub hostname: String,
    pub username: String,
    pub kdf: String,
    #[serde(rename = "N")]
    pub n: usize,
    pub r: usize,
    pub p: usize,
    pub created: DateTime<FixedOffset>,
    pub data: String,
    pub salt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoCreation {
    pub create: bool,
}
