use std::{
    fs,
    path::{Path, PathBuf},
    env,
};

use glob::glob;
use serde_derive::Deserialize;
use toml;
use hyper::{Client, Method, Request, header};
use hyper::client::HttpConnector;
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
        r##":white_check_mark: *NEW PUSH*
name: {}
URL: {}
type: {}
repository root: {}
"##,
        config.app.name, config.app.url, config.app.typename, config.app.repo
    ))
}

/// post to slack using webhook
async fn slack(url: String, message: Vec<String>) {
    let tls = hyper_rustls::HttpsConnector::with_native_roots();
    let client = Client::builder().build::<_, hyper::Body>(tls);
    let message = format!("{{text: \"{}\"}}", message[0]); // todo! multiple apps
    dbg!(&message);
    let req = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header(header::CONTENT_TYPE, "application/json")
        .body(message.into())
        .expect("request builder creation failed");
    let res = client.request(req).await;
    dbg!(res);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut message = vec![];
    let url = env::var("HICODER_ONEPAAS_SLACK_TOKEN")?;
    // dbg!(&url);
    let paths = check_onepaas_config();
    if let Some(paths) = paths {
        for path in paths {
            message.push(toml_to_message(path)?);
        }
    }
    slack(url, message).await;
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
