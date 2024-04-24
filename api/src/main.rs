use api::{self, create_api_router};

use std::{env, error::Error};
use axum::serve;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let port = env::var("API_PORT").unwrap_or("80".to_owned());

    let socket_addr = format!("0.0.0.0:{}", port);

    info!("Binding to {}", socket_addr);
    let listener = TcpListener::bind(socket_addr).await?;

    info!("Serving traffic");
    serve(listener, create_api_router()).await?;

    Ok(())
}