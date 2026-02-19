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

2. Set your authtoken:

   ```sh
   export NGROK_AUTHTOKEN=<token>
   ```

3. Add the following to your app:

   ```rust
   use ngrok::config::ForwarderBuilder;
   use url::Url;

   async fn forward_to_app() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
       let sess = ngrok::Session::builder()
           .authtoken_from_env()
           .connect()
           .await?;

       let mut listener = sess
           .http_endpoint()
           .listen_and_forward(Url::parse("http://localhost:8080")?)
           .await?;

       println!("Ingress established at: {:?}", listener.url());

       tokio::spawn(async move {
           let _ = listener.join().await;
       });

       Ok(())
   }
   ```

## License

MIT
