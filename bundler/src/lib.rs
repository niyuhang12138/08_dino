mod bundle;
pub use bundle::run_bundle;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn bundler_ts_should_work() -> Result<()> {
        let ret = run_bundle("fixtures/main.ts", &Default::default())?;

        println!("{}", ret);

        Ok(())
    }
}
