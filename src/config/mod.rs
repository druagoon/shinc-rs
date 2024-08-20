use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::consts::{CONFIG_DIR, PROJECT_DOT_ID};
use crate::include_template;

pub const DEFAULT_CONFIG: &str = include_template!("config/default.toml");

const CONFIG_FILENAME: &str = "config.toml";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let sources = vec![::config::File::from_str(DEFAULT_CONFIG, ::config::FileFormat::Toml)];
        let files: Vec<_> = Self::locate_config_files()
            .iter()
            .rev()
            .filter(|&x| x.exists())
            .map(|x| ::config::File::from(x.to_owned()).required(false))
            .collect();
        let cfg = ::config::Config::builder()
            .add_source(sources)
            .add_source(files)
            .build()?
            .try_deserialize::<Self>()?;
        Ok(cfg)
    }

    fn _get_local_file<T: AsRef<Path>>(p: T) -> PathBuf {
        std::env::current_dir().unwrap().join(PROJECT_DOT_ID).join(p)
    }

    fn _get_user_file<T: AsRef<Path>>(p: T) -> PathBuf {
        CONFIG_DIR.join(p)
    }

    pub fn locate_config_files() -> Vec<PathBuf> {
        vec![Self::get_local_config_file(), Self::get_user_config_file()]
    }

    pub fn get_local_config_file() -> PathBuf {
        Self::_get_local_file(CONFIG_FILENAME)
    }

    pub fn get_user_config_file() -> PathBuf {
        Self::_get_user_file(CONFIG_FILENAME)
    }

    #[allow(dead_code)]
    pub fn locate_template_files<T: AsRef<Path>>(p: T) -> Vec<PathBuf> {
        let suffix = Path::new("templates").join(p);
        vec![Self::_get_local_file(&suffix), Self::_get_user_file(&suffix)]
    }
}

#[allow(dead_code)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("load config failed"));
