use color_eyre::Result;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use models::ServerConfig;
use tracing_subscriber;

use std::env;

mod endpoints;
mod models;
mod server;

use server::*;

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
