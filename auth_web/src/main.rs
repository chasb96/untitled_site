use auth_web::*;

use std::error::Error;
use axum::serve;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Binding to 0.0.0.0:80");
    let listener = TcpListener::bind("0.0.0.0:80").await?;

    info!("Serving traffic");
    serve(listener, routes()).await?;

    Ok(())
}