use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{error, info};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_server(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Serving {:?} on addr {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    // axum router
    let router = Router::new()
        // .route("/*path", get(file_handler))
        .nest_service("/", dir_service)
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, router).await.unwrap();

    Ok(())
}

// 使用 pattern match
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);

    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        );
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                error!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

// 不使用 pattern match ，写法比较啰嗦
async fn hello(state: State<Arc<HttpServeState>>) -> String {
    format!("hello {:?}", state.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::Path, path::PathBuf, sync::Arc};

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });

        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;

        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
