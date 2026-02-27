# rust-sdk-example

This example application starts a hello world HTTP server on port 8085 and then uses the [ngrok Rust SDK](https://github.com/ngrok/ngrok-rust) (`ngrok` crate) to forward public traffic to that server. See the [quickstart](https://ngrok.com/docs/getting-started/rust/) and [SDK reference](https://docs.rs/ngrok/latest/ngrok/) for more details. When you run it, you'll get a public URL that anyone can use to access your app.

## Option 1: Clone and Run This Example

```sh
git clone git@github.com:ngrok/rust-sdk-example.git
cd rust-sdk-example
NGROK_AUTHTOKEN=<token> cargo run
```

## Option 2: Add ngrok to Your Own App

1. Install the SDK:

   ```sh
   cargo add ngrok
   ```

2. Add the following to your app:

   ```rust
   use ngrok::config::ForwarderBuilder;
   use ngrok::prelude::*;
   use url::Url;

   async fn connect_ngrok() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
       let sess = ngrok::Session::builder()
           .authtoken_from_env()
           .connect()
           .await?;

       let mut listener = sess
           .http_endpoint()
           .listen_and_forward(Url::parse("http://localhost:8085")?)
           .await?;

       println!("Available at: {:?}", listener.url());

       let _ = listener.join().await;
       Ok(())
   }
   ```

3. Set your authtoken and restart your app:

   ```sh
   export NGROK_AUTHTOKEN=<token>
   ```

## License

MIT
