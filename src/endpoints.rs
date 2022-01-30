use tracing::info;
use warp::hyper::Response;
use warp::{Buf, Reply};

use std::fs;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::models::{Object, RepoCreation, ServerConfig};

const LAYOUT: [&str; 5] = ["data", "index", "keys", "snapshots", "locks"];

pub async fn create_repo(create_flag: RepoCreation, server_config: ServerConfig) -> impl Reply {
    info!("Received repo creation request! Creating layouts...");
    match create_flag.create {
        true => {
            for dir in LAYOUT {
                let location = format!("{}/{}", server_config.repo_location, dir);
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

pub async fn delete_repo(server_config: ServerConfig) -> impl Reply {
    let location = format!("{}", server_config.repo_location);
    match fs::remove_dir(location) {
        Ok(_) => Response::builder().status(200).body("").unwrap(),
        Err(_) => Response::builder().status(403).body("").unwrap(),
    }
}

pub async fn create_config(mut config: impl Buf, server_config: ServerConfig) -> impl Reply {
    info!("Received config creation request! Creating config...");
    let file = fs::File::create(format!("{}/config", server_config.repo_location)).unwrap();
    let mut buf_writer = BufWriter::new(file);
    while config.has_remaining() {
        let bytes = config.chunk();
        let len = bytes.len();
        buf_writer.write(bytes).unwrap();
        config.advance(len);
    }
    Response::builder().status(200).body("").unwrap()
}

pub async fn check_config(server_config: ServerConfig) -> impl Reply {
    info!("Received config check request! Checking config file if its there...");
    match fs::metadata(format!("{}/config", server_config.repo_location)) {
        Ok(m) => Response::builder()
            .status(200)
            .header("Content-Length", m.len())
            .body("")
            .unwrap(),
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}

pub async fn get_config(server_config: ServerConfig) -> impl Reply {
    match fs::read(format!("{}/config", server_config.repo_location)) {
        Ok(c) => Response::builder().status(200).body(c).unwrap(),
        Err(_) => Response::builder().status(404).body(vec![]).unwrap(),
    }
}

pub async fn create_obj(
    obj_type: String,
    name: String,
    mut data: impl Buf,
    server_config: ServerConfig,
) -> impl Reply {
    let location = format!("{}/{}/{}", server_config.repo_location, obj_type, name);
    let file = fs::File::create(location).unwrap();
    let mut buf_writer = BufWriter::new(file);
    while data.has_remaining() {
        let bytes = data.chunk();
        let len = bytes.len();
        buf_writer.write(bytes).unwrap();
        data.advance(len);
    }
    warp::reply()
}

pub async fn get_obj_list(obj_type: String, server_config: ServerConfig) -> impl Reply {
    info!("Received object list request! Listing object...");
    let location = format!("{}/{}", server_config.repo_location, obj_type);
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

pub async fn check_obj(obj_type: String, name: String, server_config: ServerConfig) -> impl Reply {
    info!("Received object check request! Checking object...");
    let location = format!("{}/{}/{}", server_config.repo_location, obj_type, name);
    match fs::metadata(location) {
        Ok(m) => Response::builder()
            .status(200)
            .header("Content-Length", m.len())
            .body("")
            .unwrap(),
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}

pub async fn get_obj(obj_type: String, name: String, server_config: ServerConfig) -> impl Reply {
    info!("Received object get request! Getting object...");
    let location = format!("{}/{}/{}", server_config.repo_location, obj_type, name);
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

pub async fn delete_obj(obj_type: String, name: String, server_config: ServerConfig) -> impl Reply {
    info!("Received object deletion request! Deleting object...");
    let location = format!("{}/{}/{}", server_config.repo_location, obj_type, name);
    match fs::File::open(location.clone()) {
        Ok(_) => {
            fs::remove_file(location).unwrap();
            Response::builder().status(200).body("").unwrap()
        }
        Err(_) => Response::builder().status(404).body("").unwrap(),
    }
}
