use framework::{setup_server, ServerOptions};

mod routes;

#[tokio::main]
async fn main() -> tide::Result<()> {
    setup_server(ServerOptions {
        to_expose: true,
        exposed_port: Some(3000),
        bulk_routes: Some(routes::get_routes()),
    })
    .await?;

    Ok(())
}
