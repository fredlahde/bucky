use std::io::{Read, Result};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupConfig {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub exclude_git_ignore: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub interval: u32,
    pub backups: Vec<BackupConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval: 60,
            backups: vec![],
        }
    }
}

impl Config {
    pub fn read(r: &mut dyn Read) -> Result<Config> {
        let config: Config = serde_json::from_reader(r)?;
        Ok(config)
    }

    /// Tries to read from r, swallows any error and returns default() instead
    pub fn read_or_default(r: &mut dyn Read) -> Config {
        match Config::read(r) {
            Ok(c) => c,
            Err(_) => Config::default(), // TODO log
        }
    }
}
