use std::{
    fs,
    io::{BufWriter, Write},
};

use tracing::info;
use warp::hyper::body::Bytes;

use crate::models::Keys;

pub fn create_key(name: String, data: Bytes) {
    info!("Received key creation request! Creating key...");
    let keys: Keys = serde_json::from_slice(&data).unwrap();
    let location = format!("./restic/keys/{}", name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer
        .write(serde_json::to_string(&keys).unwrap().as_bytes())
        .unwrap();
}
