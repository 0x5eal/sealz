#![feature(associated_type_bounds)]

use framework::{setup_server, ServerOptions, Server};

mod api;
pub mod utils;

pub async fn init() -> tide::Result<Server> {
    let server = setup_server(ServerOptions {
        to_expose: false,
        exposed_port: None,
        bulk_routes: Some(api::get_routes_default()),
        scope: "backend",
    })
    .await?;

    Ok(server)
}
