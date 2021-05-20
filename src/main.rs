use std::{error::Error, path::{Path, PathBuf}};

use glob::{ glob};

/// check `.onepaas/workflows/*.toml` directory
/// if exists, return Option<Vec>
fn check_onepaas_config() -> Option<Vec<PathBuf>> {
    if Path::new("./.onepaas/workflows").exists() {
        let mut paths = vec![];
        for entry in glob("./.onepaas/workflows/*.toml").unwrap().filter_map(Result::ok) {
            paths.push(entry);
        }
        return Some(paths);
    }
    None
}

fn main() {
    let paths = check_onepaas_config();
}

#[cfg(test)]
mod tests {
    use crate::check_onepaas_config;
    use std::path::Path;

    #[test]
    fn test_check_onepaas_config() {
        let paths = check_onepaas_config().expect("No toml file");
        let accept = vec![Path::new(".onepaas/workflows/main.toml")];
        let fail = vec![Path::new(".onepaas/workflows/bad.toml")];
        assert_eq!(paths, accept);
        assert_ne!(paths, fail);
    }
}