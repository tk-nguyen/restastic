use tracing::info;
use warp::hyper::Response;
use warp::{hyper::body::Bytes, Reply};

use std::fs;
use std::io::{BufWriter, Write};

use crate::helpers::*;
use crate::models::RepoCreation;

const LAYOUT: [&str; 4] = ["data", "index", "keys", "snapshots"];

pub fn create_repo(create_flag: RepoCreation) -> impl Reply {
    info!("Received repo creation request! Creating layouts...");
    match create_flag.create {
        true => LAYOUT.iter().for_each(|&d| {
            let directory = format!("./restic/{}", d);
            fs::create_dir(directory).unwrap()
        }),
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
        Err(_) => Response::builder().status(200).body("").unwrap(),
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
            warp::reply()
        }
        _ => warp::reply(),
    }
}
