mod cli;
mod utils;

pub use cli::Opts;
pub(crate) use utils::*;

use cli::*;
use enum_dispatch::enum_dispatch;

const BUILD_DIR: &str = ".build";

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn dino_should_work() {}
}
