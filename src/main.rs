use std::{
    fs,
    path::{Path, PathBuf},
};

use glob::glob;
use serde_derive::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    app: App,
}
#[derive(Debug, Deserialize)]
struct App {
    name: String,
    url: String,
    #[serde(alias = "type")]
    typename: String,
    repo: String,
}

/// check `.onepaas/workflows/*.toml` directory
/// if exists, return Option<Vec>
fn check_onepaas_config() -> Option<Vec<PathBuf>> {
    if Path::new("./.onepaas/workflows").exists() {
        let mut paths = vec![];
        for entry in glob("./.onepaas/workflows/*.toml")
            .unwrap()
            .filter_map(Result::ok)
        {
            paths.push(entry);
        }
        return Some(paths);
    }
    None
}

/// if error, return error message
/// message {content: [{app: detail}], err: ""}
fn toml_to_message(path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let raw_config = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&raw_config)?;
    Ok(format!(
        r##"name: {}
URL: {}
type: {}
repository root: {}
"##,
        config.app.name, config.app.url, config.app.typename, config.app.repo
    ))
}

/// post to slack using webhook
fn slack(message: Vec<String>) {
    todo!()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut message = vec![];
    let paths = check_onepaas_config();
    if let Some(paths) = paths {
        for path in paths {
            message.push(toml_to_message(path)?);
        }
    }
    slack(message);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{check_onepaas_config, toml_to_message};
    use std::path::Path;

    #[test]
    fn test_check_onepaas_config() {
        let paths = check_onepaas_config().expect("No toml file");
        let accept = vec![Path::new(".onepaas/workflows/main.toml")];
        let fail = vec![Path::new(".onepaas/workflows/bad.toml")];
        assert_eq!(paths, accept);
        assert_ne!(paths, fail);
    }
    #[test]
    fn test_toml_to_message() -> Result<(), Box<dyn std::error::Error>> {
        let paths = check_onepaas_config().expect("No toml file");
        let mut config = vec![];
        let ans = vec![r##"name: sample
URL: repo.username.hicoder.one
type: bot/discord
repository root: .
"##];
        for path in paths {
            config.push(toml_to_message(path)?)
        }
        assert_eq!(config, ans);
        Ok(())
    }
}
