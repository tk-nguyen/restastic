use std::collections::HashMap;

use color_eyre::Result;
use tracing::info;
use warp::{hyper::body::Bytes, Rejection, Reply};

pub fn create_repo(map: HashMap<String, String>, test: Bytes) -> impl Reply {
    info!("Received: {:#?}", test);
    "Ok"
}

pub fn create_config(config: Bytes) -> impl Reply {
    info!("Received: {:#?}", config);
    "Ok"
}

pub fn create_type(path1: String, path2: String, types: Bytes) -> impl Reply {
    info!("Requested path: /{}/{}", path1, path2);
    info!("Received: {:#?}", types);
    "Ok"
}
