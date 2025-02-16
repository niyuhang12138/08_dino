mod build;
mod init;
mod run;

pub use build::BuildOpts;
pub use init::InitOpts;
pub use run::RunOpts;

use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Parser, Debug)]
#[command(name = "dino", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    #[command(name = "init", about = "Initialize a new project")]
    Init(InitOpts),
    #[command(name = "build", about = "Build the project")]
    Build(BuildOpts),
    #[command(name = "run", about = "Run the project")]
    Run(RunOpts),
}
