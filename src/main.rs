#[tokio::main]
async fn main() {
    let backend_service = match backend::init().await {
        Ok(server) => server,
        Err(err) => panic!(
            "[backend] :: error `{}` occurred while initializing",
            err.to_string()
        ),
    };

    backend_service
        .expose("backend", 3000)
        .await
        .expect("failed to expose backend service to port `3000`!");
}
