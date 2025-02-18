use crate::{build_project, CmdExecutor};
use clap::Parser;
use dino_server::{start_server, ProjectConfig, SwappableAppRouter, TenentRouter};
use std::{env, fs};
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt as _, util::SubscriberInitExt as _,
    Layer as _,
};

#[derive(Parser, Debug)]
pub struct RunOpts {
    // prot to listen
    #[arg(short, long, default_value = "3000")]
    pub port: u16,
}

impl CmdExecutor for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let layer = Layer::new().with_filter(LevelFilter::INFO);
        tracing_subscriber::registry().with(layer).init();

        let cur = env::current_dir()?.display().to_string();
        let filename = build_project(&cur)?;
        let code = fs::read_to_string(&filename)?;
        let config = ProjectConfig::load(filename.replace(".mjs", ".yml"))?;

        let tenent = TenentRouter::new(
            "localhost",
            SwappableAppRouter::try_new(&code, config.routes)?,
        );

        start_server(self.port, vec![tenent]).await?;

        Ok(())
    }
}
