# rust-sdk-example

A minimal HTTP server using the [ngrok Rust SDK](https://github.com/ngrok/ngrok-rust) (`ngrok` crate).

## Clone and Run

```sh
git clone git@github.com:ngrok/rust-sdk-example.git
cd rust-sdk-example
NGROK_AUTHTOKEN=<token> cargo run
```

## Add to Existing Code

1. Install the SDK:

   ```sh
   cargo add ngrok
   ```

2. Add the following to your app:

   ```rust
   use ngrok::config::ForwarderBuilder;
   use url::Url;

   async fn forward_to_app() -> Result<Forwarder<HttpTunnel>, Box<dyn std::error::Error + Send + Sync>> {
       let sess = ngrok::Session::builder()
           .authtoken_from_env()
           .connect()
           .await?;

       let forwarder = sess
           .http_endpoint()
           .listen_and_forward(Url::parse("http://localhost:8080")?)
           .await?;

       println!("Ingress established at: {:?}", forwarder.url());
       Ok(forwarder)
   }
   ```

## License

MIT
