use host::*;
use log::info;

use std::error::Error;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("Binding to 0.0.0.0:80");
    let listener = TcpListener::bind("0.0.0.0:80").await?;

    info!("Serving traffic");
    serve(listener, routes()).await?;

    Ok(())
}
