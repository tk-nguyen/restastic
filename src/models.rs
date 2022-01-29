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
pub struct Locks {
    pub time: DateTime<FixedOffset>,
    pub exclusive: bool,
    pub hostname: String,
    pub username: String,
    pub pid: usize,
    pub uid: usize,
    pub gid: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoCreation {
    pub create: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub name: String,
    pub size: usize,
}

impl Object {
    pub fn new(name: String, size: usize) -> Self {
        Object { name, size }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub repo_location: String,
}
