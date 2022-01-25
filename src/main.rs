use color_eyre::Result;
use models::RepoCreation;
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
    server_start().await;
    Ok(())
}

async fn server_start() {
    let repo_creation = warp::path::end()
        .and(warp::query::<RepoCreation>())
        .map(create_repo);
    let repo_deletion = warp::path::end().map(delete_repo);

    let config_check = warp::path("config").map(check_config);
    let config_get = warp::path("config").map(get_config);
    let config_creation = warp::path("config")
        .and(warp::body::bytes())
        .map(create_config);

    let obj_list_get = warp::path!(String).map(get_obj_list);
    let obj_check = warp::path!(String / String).map(check_obj);
    let obj_get = warp::path!(String / String).map(get_obj);
    let obj_creation = warp::path!(String / String)
        .and(warp::body::bytes())
        .map(create_obj);
    let obj_deletion = warp::path!(String / String).map(delete_obj);

    let post_routes = warp::post().and(repo_creation.or(config_creation).or(obj_creation));
    let head_routes = warp::head().and(config_check.or(obj_check));
    let get_routes = warp::get().and(config_get.or(obj_list_get).or(obj_get));
    let delete_routes = warp::delete().and(obj_deletion.or(repo_deletion));

    let api = post_routes
        .or(head_routes)
        .or(get_routes)
        .or(delete_routes)
        .with(warp::trace::request());

    warp::serve(api).run(([0, 0, 0, 0], 8000)).await;
}
