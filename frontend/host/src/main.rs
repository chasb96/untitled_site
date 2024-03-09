use axum::Router;
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest_service(
            "/", 
            ServeDir::new("./app/dist")
                .not_found_service(
                    ServeFile::new("./app/dist/index.html")
                )
        );

    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .unwrap();

    axum::serve(listener, router)
        .await
        .unwrap();
}
