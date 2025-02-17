mod config;
mod handler;
mod router;

use axum::{routing::any, Router};
use handler::handler;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug, Clone, Default)]
pub struct AppState {}

pub async fn start_server(port: u16) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    let state = AppState::new();

    let app = Router::new()
        .route("/{*path}", any(handler))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn dino_server_should_work() {}
}
