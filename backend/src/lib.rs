use framework::{setup_server, ServerOptions, Server};

mod routes;

pub async fn init() -> tide::Result<Server> {
    let server = setup_server(ServerOptions {
        to_expose: false,
        exposed_port: None,
        bulk_routes: Some(routes::get_routes()),
        scope: "backend",
    })
    .await?;

    Ok(server)
}
