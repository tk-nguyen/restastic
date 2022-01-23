use color_eyre::Result;
use std::collections::HashMap;
use tracing_subscriber;
use warp::Filter;

use std::env;

mod endpoints;
mod models;

use endpoints::*;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
    let repo_creation = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::body::bytes())
        .map(create_repo);
    let config_creation = warp::path("config")
        .and(warp::body::bytes())
        .map(create_config);
    let type_creation = warp::path!(String / String)
        .and(warp::body::bytes())
        .map(create_type);
    let post_routes = warp::post().and(repo_creation.or(config_creation).or(type_creation));
    warp::serve(post_routes)
        .run(([192, 168, 100, 11], 8000))
        .await;
    Ok(())
}
