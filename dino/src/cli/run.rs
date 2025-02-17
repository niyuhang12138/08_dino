use crate::{build_project, CmdExecutor};
use clap::Parser;
use dino_server::{JsWorker, Req};
use std::{env, fs};

#[derive(Parser, Debug)]
pub struct RunOpts {}

impl CmdExecutor for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let cur = env::current_dir()?.display().to_string();
        let filename = build_project(&cur)?;
        let content = fs::read_to_string(&filename)?;
        let worker = JsWorker::try_new(&content)?;
        // TODO: normally this should run axum and let it load the worker
        let req = Req::builder()
            .method("GET")
            .url("http://example.com")
            .build();
        let res = worker.run("hello", req)?;
        println!("response: {res:#?}");

        Ok(())
    }
}
