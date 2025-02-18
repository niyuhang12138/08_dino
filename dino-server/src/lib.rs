mod config;
mod engine;
mod error;
mod handler;
mod router;

pub use config::*;
pub use engine::*;
pub use error::AppError;
pub use router::*;

use axum::{routing::any, Router};
use dashmap::DashMap;
use handler::handler;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug, Clone)]
pub struct AppState {
    // key is hostname
    routes: DashMap<String, SwappableAppRouter>,
}

#[derive(Debug, Clone)]
pub struct TenentRouter {
    host: String,
    router: SwappableAppRouter,
}

pub async fn start_server(port: u16, routers: Vec<TenentRouter>) -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    let routers = routers
        .into_iter()
        .map(|t| (t.host, t.router))
        .collect::<DashMap<_, _>>();

    let state = AppState::new(routers);

    let app = Router::new()
        .route("/{*path}", any(handler))
        .with_state(state);

    axum::serve(listener, app).await?;

    Ok(())
}

impl AppState {
    pub fn new(router: DashMap<String, SwappableAppRouter>) -> Self {
        Self { routes: router }
    }
}

impl TenentRouter {
    pub fn new(host: impl Into<String>, router: SwappableAppRouter) -> Self {
        Self {
            host: host.into(),
            router,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn dino_server_should_work() {}
}
