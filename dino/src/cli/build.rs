use std::env;

use crate::{build_project, CmdExecutor};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct BuildOpts {}

impl CmdExecutor for BuildOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let cur = env::current_dir()?.display().to_string();
        let filename = build_project(&cur)?;
        println!("Build success: {filename}");
        Ok(())
    }
}
