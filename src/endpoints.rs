use tracing::info;
use warp::hyper::Response;
use warp::{hyper::body::Bytes, Reply};

use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};

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
    Response::builder()
        .status(200)
        .header("Content-Type", "application/vnd.x.restic.rest.v2")
        .body("")
        .unwrap()
}

pub fn delete_repo() -> impl Reply {
    let location = "./restic";
    match fs::remove_dir(location) {
        Ok(_) => Response::builder().status(200).body("").unwrap(),
        Err(_) => Response::builder().status(403).body("").unwrap(),
    }
}

pub fn create_config(config: Bytes) -> impl Reply {
    info!("Received config creation request! Creating config...");
    let file = fs::File::create("./restic/config").unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&config).unwrap();
    Response::builder().status(200).body("").unwrap()
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
    let location = format!("./restic/{}/{}", obj_type, name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(&data).unwrap();
    warp::reply()
}

pub fn get_obj_list(obj_type: String) -> impl Reply {
    info!("Received object list request! Listing object...");
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

pub fn check_obj(obj_type: String, name: String) -> impl Reply {
    info!("Received object check request! Checking object...");
    let location = format!("./restic/{}/{}", obj_type, name);
    match fs::metadata(location) {
        Ok(m) => Response::builder()
            .status(200)
            .header("Content-Length", m.len())
            .body("")
            .unwrap(),
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}

pub fn get_obj(obj_type: String, name: String) -> impl Reply {
    info!("Received object get request! Getting object...");
    let location = format!("./restic/{}/{}", obj_type, name);
    match fs::File::open(location) {
        Ok(f) => {
            let mut buf_reader = BufReader::new(f);
            let mut buf = vec![];
            buf_reader.read_to_end(&mut buf).unwrap();
            Response::builder().status(200).body(buf).unwrap()
        }
        Err(_) => Response::builder().status(404).body(vec![]).unwrap(),
    }
}

pub fn delete_obj(obj_type: String, name: String) -> impl Reply {
    info!("Received object deletion request! Deleting object...");
    let location = format!("./restic/{}/{}", obj_type, name);
    match fs::File::open(location.clone()) {
        Ok(_) => {
            fs::remove_file(location).unwrap();
            Response::builder().status(200).body("").unwrap()
        }
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}
