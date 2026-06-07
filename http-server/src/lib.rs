mod handler;
mod mime;
mod request;
mod response;
mod server;

use std::net::SocketAddr;

pub use server::run;

pub async fn serve(port: u16, serve_dir: &str) -> anyhow::Result<()> {
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    run(addr, serve_dir.to_string()).await
}
