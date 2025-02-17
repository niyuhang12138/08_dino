use dino_server::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    start_server(4923).await?;

    Ok(())
}
