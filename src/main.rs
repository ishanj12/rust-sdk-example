use axum::{routing::get, Router};
use ngrok::config::ForwarderBuilder;
use ngrok::prelude::*;
use std::net::SocketAddr;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new().route("/", get(|| async { "Hello from ngrok-rust!\n" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tokio::spawn(async move {
        axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
            .await
            .unwrap();
    });

    let sess = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?;

    let listener = sess
        .http_endpoint()
        .listen_and_forward(Url::parse("http://localhost:8080")?)
        .await?;

    println!("Ingress established at: {:?}", listener.url());

    tokio::signal::ctrl_c().await?;
    Ok(())
}
