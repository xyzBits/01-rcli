use anyhow::Result;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::Path;
use tokio::net::TcpListener;
use tracing::info;

pub fn process_http_server(path: &Path, port: u16) -> Result<()> {
    info!("Serving {:?} on port {}", path, port);

    // axum router
    let router = Router::new().route("/", get(index_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // TcpListener::bind(addr).await

    Ok(())
}

async fn index_handler() {
    "hello world"
}
