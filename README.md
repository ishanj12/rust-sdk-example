# rust-sdk-example

This example application starts a hello world HTTP server on port 8085 and then uses the [ngrok Rust SDK](https://github.com/ngrok/ngrok-rust) (`ngrok` crate) to forward public traffic to that server. See the [quickstart](https://ngrok.com/docs/getting-started/rust/) and [SDK reference](https://docs.rs/ngrok/latest/ngrok/) for more details. When you run it, you'll get a public URL that anyone can use to access your app.

## Clone and Run This Example

```sh
git clone git@github.com:ngrok/rust-sdk-example.git
cd rust-sdk-example
NGROK_AUTHTOKEN=<token> cargo run
```

## License

MIT
