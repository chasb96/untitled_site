mod log;
mod health;

use axum::routing::get;
use axum::Router;
use ::log::error;
use ::log::info;
use log::LogLayer;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Registering Service Directory");
    let router = Router::new()
        .route("/health", get(health::health))
        .nest_service(
            "/", 
            ServeDir::new("./app/dist")
                .not_found_service(
                    ServeFile::new("./app/dist/index.html")
                )
        )
        .layer(LogLayer::new());

    info!("Binding to 0.0.0.0:80");
    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();

    info!("Serving traffic on 0.0.0.0:80");
    axum::serve(listener, router)
        .await
        .inspect_err(|e| error!("{:?}", e))
        .unwrap();
}
