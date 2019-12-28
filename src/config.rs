use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize)]
enum RemotablePath {
    Local(PathBuf),
    Remote(String), // TODO: Implement me
}

impl From<&Path> for RemotablePath {
    fn from(p: &Path) -> Self {
        RemotablePath::Local(p.to_owned())
    }
}

impl From<&str> for RemotablePath {
    fn from(_: &str) -> Self {
        unimplemented! {}
    }
}
#[derive(Deserialize)]
struct BackupConfig {
    source: RemotablePath,
    dest: RemotablePath,
}

#[derive(Deserialize)]
pub struct Config {
    interval: Duration,
    backups: Vec<BackupConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval: Duration::from_secs(60),
            backups: vec![],
        }
    }
}

impl Config {
    pub fn read(r: &mut dyn Read) -> Result<Config> {
        let config: Config = serde_json::from_reader(r)?;
        Ok(config)
    }

    /// Tries to read from r, swallow any error and returns default() instead
    pub fn read_or_default(r: &mut dyn Read) -> Config {
        match Config::read(r) {
            Ok(c) => c,
            Err(_) => Config::default(), // TODO log
        }
    }
}
