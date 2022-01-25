use tracing::info;
use warp::hyper::Response;
use warp::{hyper::body::Bytes, Reply};

use std::fs;
use std::io::{BufWriter, Write};

use crate::helpers::*;
use crate::models::{Object, RepoCreation};

const LAYOUT: [&str; 5] = ["data", "index", "keys", "snapshots", "locks"];

pub fn create_repo(create_flag: RepoCreation) -> impl Reply {
    info!("Received repo creation request! Creating layouts...");
    match create_flag.create {
        true => {
            for dir in LAYOUT {
                let location = format!("./restic/{}", dir);
                fs::create_dir(location).unwrap();
            }
        }
        false => (),
    }
    warp::reply::with_header(
        warp::reply(),
        "Content-Type",
        "application/vnd.x.restic.rest.v2",
    )
}

pub fn create_config(config: Bytes) -> impl Reply {
    info!("Received config creation request! Creating config...");
    let file = fs::File::create("./restic/config").unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&config).unwrap();
    warp::reply()
}

pub fn check_config() -> impl Reply {
    info!("Received config check request! Checking config file if its there...");
    match fs::metadata("./restic/config") {
        Ok(m) => Response::builder()
            .status(200)
            .header("Content-Length", m.len())
            .body("")
            .unwrap(),
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}

pub fn get_config() -> impl Reply {
    match fs::read("./restic/config") {
        Ok(c) => Response::builder().status(200).body(c).unwrap(),
        Err(_) => Response::builder().status(404).body(vec![]).unwrap(),
    }
}

pub fn create_obj(obj_type: String, name: String, data: Bytes) -> impl Reply {
    match obj_type.as_str() {
        "keys" => {
            create_key(name, data);
            Response::builder().status(200).body("").unwrap()
        }
        "locks" => {
            create_lock(name, data);
            Response::builder().status(200).body("").unwrap()
        }
        "data" => {
            create_data(name, data);
            Response::builder().status(200).body("").unwrap()
        }
        "index" => {
            create_index(name, data);
            Response::builder().status(200).body("").unwrap()
        }
        "snapshots" => {
            create_snapshot(name, data);
            Response::builder().status(200).body("").unwrap()
        }
        _ => unimplemented!(),
    }
}

pub fn get_obj_list(obj_type: String) -> impl Reply {
    info!("Received object list request! Listing obj...");
    let location = format!("./restic/{}", obj_type);
    match fs::read_dir(location) {
        Ok(f) => {
            let mut response = vec![];
            for file in f {
                let file = file.unwrap();
                let obj = Object::new(
                    file.file_name().into_string().unwrap(),
                    file.metadata().unwrap().len() as usize,
                );
                response.push(obj);
            }
            Response::builder()
                .status(200)
                .header("Content-Type", "application/vnd.x.restic.rest.v2")
                .body(serde_json::to_string(&response).unwrap())
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(404)
            .body("".to_string())
            .unwrap(),
    }
}

pub fn get_obj(obj_type: String, name: String) -> impl Reply {
    info!("Received object get request! Getting obj...");
    match obj_type.as_str() {
        "keys" => {
            let key = get_key(name);
            Response::builder()
                .status(200)
                .header("Content-Type", "binary/octet-stream")
                .body(key)
                .unwrap()
        }
        "locks" => {
            let lock = get_lock(name);
            Response::builder()
                .status(200)
                .header("Content-Type", "binary/octet-stream")
                .body(lock)
                .unwrap()
        }
        _ => unimplemented!(),
    }
}

pub fn delete_obj(obj_type: String, name: String) -> impl Reply {
    info!("Received object deletion request! Deleting object...");
    match obj_type.as_str() {
        "locks" => {
            delete_lock(name);
            Response::builder().status(200).body("").unwrap()
        }
        "snapshots" => {
            delete_snapshot(name);
            Response::builder().status(200).body("").unwrap()
        }

        _ => unimplemented!(),
    }
}
