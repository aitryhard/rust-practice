use crate::handler;
use crate::request;
use crate::response::Response;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run(addr: SocketAddr, serve_dir: String) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Serving {} on http://{}", serve_dir, addr);

    loop {
        let (mut socket, peer) = listener.accept().await?;
        let dir = serve_dir.clone();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 8192];
            let n = match socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => n,
                Err(_) => return,
            };

            let raw = String::from_utf8_lossy(&buf[..n]);

            let response = match request::parse(&raw) {
                Ok(req) => {
                    println!("{} {} {} — {}", peer, req.method, req.path, 200);
                    handler::handle(&req, &dir).await
                }
                Err(e) => {
                    println!("{} — 400 ({})", peer, e);
                    Response::new(400, "text/plain", b"Bad Request".to_vec())
                }
            };

            let _ = socket.write_all(&response.to_bytes()).await;
        });
    }
}
