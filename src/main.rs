use color_eyre::Result;
use models::RepoCreation;
use tracing_subscriber;
use warp::Filter;

use std::env;

mod endpoints;
mod helpers;
mod models;

use endpoints::*;

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::init();
    let repo_creation = warp::path::end()
        .and(warp::query::<RepoCreation>())
        .map(create_repo);
    let config_creation = warp::path("config")
        .and(warp::body::bytes())
        .map(create_config);
    let config_check = warp::path("config").map(check_config);
    let config_get = warp::path("config").map(get_config);
    let obj_creation = warp::path!(String / String)
        .and(warp::body::bytes())
        .map(create_obj);
    let post_routes = warp::post().and(repo_creation.or(config_creation).or(obj_creation));
    let head_routes = warp::head().and(config_check);
    let get_routes = warp::get().and(config_get);

    let api = post_routes
        .or(head_routes)
        .or(get_routes)
        .with(warp::trace::request());

    warp::serve(api).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
