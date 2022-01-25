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

pub fn create_lock(name: String, data: Bytes) {
    info!("Received lock creation request! Creating lock...");
    let location = format!("./restic/locks/{}", name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&data).unwrap();
}

pub fn create_data(name: String, data: Bytes) {
    info!("Received data creation request! Creating data...");
    let location = format!("./restic/data/{}", name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&data).unwrap();
}

pub fn create_index(name: String, data: Bytes) {
    info!("Received index creation request! Creating index...");
    let location = format!("./restic/index/{}", name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&data).unwrap();
}

pub fn create_snapshot(name: String, data: Bytes) {
    info!("Received snapshot creation request! Creating snapshot...");
    let location = format!("./restic/snapshots/{}", name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&data).unwrap();
}

pub fn get_key(name: String) -> Vec<u8> {
    info!("Received key get request! Getting key...");
    let location = format!("./restic/keys/{}", name);
    let key = fs::read(location).unwrap();
    key
}

pub fn get_lock(name: String) -> Vec<u8> {
    info!("Received lock get request! Getting lock...");
    let location = format!("./restic/locks/{}", name);
    let lock = fs::read(location).unwrap();
    lock
}

pub fn delete_lock(name: String) {
    info!("Received lock deletion request! Deleting lock...");
    let location = format!("./restic/locks/{}", name);
    fs::remove_file(location).unwrap()
}

pub fn delete_snapshot(name: String) {
    info!("Received snapshot deletion request! Deleting snapshot...");
    let location = format!("./restic/locks/{}", name);
    fs::remove_file(location).unwrap()
}
