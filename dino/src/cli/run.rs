use crate::{build_project, CmdExecutor};
use clap::Parser;
use dino_server::{start_server, ProjectConfig, SwappableAppRouter, TenentRouter};
use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use std::{fs, path::Path, time::Duration};
use tokio::sync::mpsc::channel;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt as _, util::SubscriberInitExt as _,
    Layer as _,
};

const MONITOR_FS_INTERVAL: Duration = Duration::from_secs(2);

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

        // let cur = env::current_dir()?.display().to_string();
        let (code, config) = get_code_and_config()?;

        let router = SwappableAppRouter::try_new(&code, config.routes)?;
        let tenent = TenentRouter::new("localhost", router.clone());

        tokio::spawn(watch_project(".", router));

        start_server(self.port, vec![tenent]).await?;

        Ok(())
    }
}

fn get_code_and_config() -> anyhow::Result<(String, ProjectConfig)> {
    let filename = build_project(".")?;
    let code = fs::read_to_string(&filename)?;
    let config = ProjectConfig::load(filename.replace(".mjs", ".yml"))?;
    Ok((code, config))
}

#[allow(unused_assignments)]
/// listen to file changes and reload the router
async fn watch_project(dir: &'static str, router: SwappableAppRouter) -> anyhow::Result<()> {
    let (tx, rx) = channel(1);

    let mut debouncer = new_debouncer(MONITOR_FS_INTERVAL, None, move |res| {
        tx.blocking_send(res).unwrap()
    })?;

    let path = Path::new(dir);

    debouncer.watch(path, RecursiveMode::Recursive)?;

    let mut stream = ReceiverStream::new(rx);

    while let Some(ret) = stream.next().await {
        match ret {
            Ok(events) => {
                let mut need_swap = false;

                for event in events {
                    // warn!("event: {:?}", event);
                    need_swap = event.paths.iter().any(|p| is_ts_or_js_or_config_toml(p));

                    if !need_swap {
                        continue;
                    }

                    info!("reloading content...");
                    let (code, config) = get_code_and_config()?;
                    router.swap(code, config.routes)?;
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }

    Ok(())
}

/// 判断数组中是否有PathBuf的后缀名是否包含.ts或者.js或者config.toml文件
fn is_ts_or_js_or_config_toml(path: &Path) -> bool {
    let ext = path.extension().unwrap_or_default();
    ext == "ts" || ext == "js" || path.file_name().unwrap() == "config.toml"
}
