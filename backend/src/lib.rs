#![feature(associated_type_bounds)]
#![feature(drain_filter)]

use framework::{setup_server, ServerOptions, Server};

mod api;

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
