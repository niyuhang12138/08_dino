use std::{fs, path::Path};

use crate::CmdExecutor;
use anyhow::Result;
use askama::Template;
use clap::Parser;
use dialoguer::Input;
use git2::Repository;

#[derive(Template)]
#[template(path = "config.yml.j2")]
struct ConfigYmlFile {
    name: String,
}

#[derive(Template)]
#[template(path = "main.ts.j2")]
struct MainTsFile {}

#[derive(Template)]
#[template(path = ".gitignore.j2")]
struct GitIgnoreFile {}

#[derive(Parser, Debug)]
pub struct InitOpts {}

impl CmdExecutor for InitOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let name: String = Input::new().with_prompt("<Project Name>").interact_text()?;

        // if current dir is empty then init project, otherwise create new dir and init project
        let cur = Path::new(".");
        if fs::read_dir(".")?.next().is_none() {
            init_project(&name, cur)?;
        } else {
            let path = cur.join(&name);
            init_project(&name, &path)?;
        }

        Ok(())
    }
}

fn init_project(project_name: &str, path: &Path) -> Result<()> {
    Repository::init(path)?;

    // init config file
    let config = ConfigYmlFile {
        name: project_name.to_string(),
    };
    fs::write(path.join("config.yml"), config.render()?)?;

    // init main.ts file
    fs::write(path.join("main.ts"), MainTsFile {}.render()?)?;

    // init .gitignore file
    fs::write(path.join(".gitignore"), GitIgnoreFile {}.render()?)?;

    Ok(())
}
