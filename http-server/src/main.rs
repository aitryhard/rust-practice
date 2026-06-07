use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let port: u16 = match args.get(1) {
        Some(p) => p.parse().unwrap_or(8080),
        None => 8080,
    };

    let serve_dir = args.get(2).cloned().unwrap_or_else(|| "public".to_string());

    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    http_server::run(addr, serve_dir).await
}
