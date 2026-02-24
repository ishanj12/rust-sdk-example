use axum::{routing::get, Router};
use ngrok::config::ForwarderBuilder;
use ngrok::prelude::*;
use std::net::SocketAddr;
use url::Url;

// This HTTP server is just for demonstration. If you already have an app
// running, skip start_server() and point ngrok at its port instead.
async fn start_server() {
    let app = Router::new().route("/", get(|| async { "Hello from ngrok-rust!\n" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8085));
    tokio::spawn(async move {
        axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
            .await
            .unwrap();
    });
    println!("Server listening on port 8085");
}

async fn connect_ngrok() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sess = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await?;

    let mut listener = sess
        .http_endpoint()

        // Uncomment below to use a specific domain.
        // https://dashboard.ngrok.com/domains
        // .domain("<your_domain_here>")

        // Uncomment below to load balance across multiple instances of your app.
        // https://ngrok.com/docs/universal-gateway/endpoint-pooling/
        // .pooling_enabled(true)

        // Uncomment below to require visitors to log in with Google before accessing your app.
        // https://ngrok.com/docs/traffic-policy/actions/oauth/
        // .traffic_policy(r#"{"on_http_request":[{"actions":[{"type":"oauth","config":{"provider":"google"}}]}]}"#)

        .listen_and_forward(Url::parse("http://localhost:8085")?)
        .await?;

    println!("Available at: {:?}", listener.url());

    let _ = listener.join().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_server().await;
    connect_ngrok().await
}
