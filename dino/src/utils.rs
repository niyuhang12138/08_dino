use crate::BUILD_DIR;
use anyhow::Result;
use bundler::run_bundle;
use glob::{glob, GlobError};
use std::{
    collections::BTreeSet,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

// get all files with certain extension in a directory
pub(crate) fn get_files_with_extension(dir: &str, exts: &[&str]) -> Result<BTreeSet<PathBuf>> {
    let mut all_files = BTreeSet::new();
    for ext in exts {
        let rule = format!("{dir}/**/*.{ext}");
        let files = glob(&rule)?.collect::<Result<BTreeSet<PathBuf>, GlobError>>()?;
        all_files.extend(files);
    }
    Ok(all_files)
}

pub(crate) fn calc_hash_for_project(dir: &str) -> Result<String> {
    calc_hash_for_files(dir, &["ts", "json"], 16)
}

pub(crate) fn calc_hash_for_files(dir: &str, exts: &[&str], len: usize) -> Result<String> {
    let files = get_files_with_extension(dir, exts)?;
    let mut hasher = blake3::Hasher::new();
    for file in files {
        hasher.update_reader(fs::File::open(file)?)?;
    }

    let mut ret = hasher.finalize().to_string();
    ret.truncate(len);

    Ok(ret)
}

pub(crate) fn build_project(dir: &str) -> Result<String> {
    let hash = calc_hash_for_project(dir)?;

    fs::create_dir_all(BUILD_DIR)?;

    let filename = format!("{BUILD_DIR}/{hash}.mjs");

    let config = format!("{BUILD_DIR}/{hash}.yml");

    let dst: &Path = Path::new(&filename);

    // if the file already exists, skip the build
    if dst.exists() {
        return Ok(filename);
    }

    // build the project
    let content = run_bundle("main.ts", &Default::default())?;

    fs::write(dst, content)?;

    let mut dst = File::create(&config)?;
    let mut src = File::open("config.yml")?;

    io::copy(&mut src, &mut dst)?;

    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_files_with_extension_should_work() -> Result<()> {
        let files = get_files_with_extension("fixtures/prj", &["ts", "js", "json"])?;

        assert_eq!(files.len(), 3);
        assert_eq!(
            files.into_iter().collect::<Vec<_>>(),
            [
                PathBuf::from("fixtures/prj/main.ts"),
                PathBuf::from("fixtures/prj/utils/fetch.json"),
                PathBuf::from("fixtures/prj/utils/math.js"),
            ]
        );

        Ok(())
    }

    #[test]
    fn calc_hash_for_files_should_work() -> Result<()> {
        let hash = calc_hash_for_files("fixtures/prj", &["ts", "js", "json"], 12)?;

        assert_eq!(hash, "af1349b9f5f9");

        Ok(())
    }
}
