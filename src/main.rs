use color_eyre::Result;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use models::{RepoCreation, ServerConfig};
use tracing::error;
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
    // We load the config first
    // then pass that to handlers
    let server_config = Figment::new()
        .merge(Toml::file("./src/Settings.toml"))
        .extract::<ServerConfig>()?;
    server_start(server_config).await;

    Ok(())
}

async fn server_start(config: ServerConfig) {
    // Check if the repo location exit
    // Immediately quit the server if it does not
    match std::fs::metadata(config.clone().repo_location) {
        Ok(_) => {
            let repo_creation = warp::path::end()
                .and(warp::query::<RepoCreation>())
                .and(with_server_config(config.clone()))
                .then(create_repo);
            let repo_deletion = warp::path::end()
                .and(with_server_config(config.clone()))
                .then(delete_repo);

            let config_check = warp::path("config")
                .and(with_server_config(config.clone()))
                .then(check_config);
            let config_get = warp::path("config")
                .and(with_server_config(config.clone()))
                .then(get_config);
            let config_creation = warp::path("config")
                .and(warp::body::bytes())
                .and(with_server_config(config.clone()))
                .then(create_config);

            let obj_list_get = warp::path!(String)
                .and(with_server_config(config.clone()))
                .then(get_obj_list);
            let obj_check = warp::path!(String / String)
                .and(with_server_config(config.clone()))
                .then(check_obj);
            let obj_get = warp::path!(String / String)
                .and(with_server_config(config.clone()))
                .then(get_obj);
            let obj_creation = warp::path!(String / String)
                .and(warp::body::bytes())
                .and(with_server_config(config.clone()))
                .then(create_obj);
            let obj_deletion = warp::path!(String / String)
                .and(with_server_config(config.clone()))
                .then(delete_obj);

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
        Err(_) => {
            error!("No such directory exist: `{}`", config.repo_location)
        }
    }
}
