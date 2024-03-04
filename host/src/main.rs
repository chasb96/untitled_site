use host::*;

use std::error::Error;
use tokio::net::TcpListener;
use axum::serve;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:80").await?;

    serve(listener, routes()).await?;

    Ok(())
}
