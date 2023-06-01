use once_cell::unsync::Lazy;
use tracing::Level;

#[tokio::main]
async fn main() {
    let tracing_level: Lazy<Level> = Lazy::new(|| {
        if cfg!(debug_assertions) {
            Level::TRACE
        } else {
            Level::INFO
        }
    });

    tracing_subscriber::fmt()
        .with_max_level((&*tracing_level).to_owned())
        .init();

    better_panic::install();

    let backend_service = match backend::init().await {
        Ok(server) => server,
        Err(err) => {
            tracing::error!(target: "backend_service", "error `{}` occurred while initializing", err.to_string());

            std::process::exit(1);
        }
    };

    backend_service
        .expose("backend", 3000)
        .await
        .expect("failed to expose backend service to port `3000`!");
}
