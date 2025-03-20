use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::include_template;
use crate::utils::path::expand_tilde;

/// The default configuration template.
pub const DEFAULT_CONFIG: &str = include_template!("config/default.toml");

/// The sub-directory where local or user configuration files are stored.
pub const CONFIG_SUBDIR: &str = concat!(".config/", clap::crate_name!());
/// The name of the configuration file.
pub const CONFIG_FILENAME: &str = "config.toml";
/// The directory where user configuration files are stored.
pub static USER_CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let home = expand_tilde("~");
    home.join(CONFIG_SUBDIR)
});

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        let sources = vec![::config::File::from_str(DEFAULT_CONFIG, ::config::FileFormat::Toml)];
        let files: Vec<_> = Self::locate_config_paths()
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

    /// Resolve the path of configuration file that stored in `current_dir`.
    fn resolve_local_path<T: AsRef<Path>>(p: T) -> PathBuf {
        std::env::current_dir().unwrap().join(CONFIG_SUBDIR).join(p)
    }

    /// Resolve the path of configuration file that stored in `USER_CONFIG_DIR`.
    fn resolve_user_path<T: AsRef<Path>>(p: T) -> PathBuf {
        USER_CONFIG_DIR.join(p)
    }

    pub fn local_config_path() -> PathBuf {
        Self::resolve_local_path(CONFIG_FILENAME)
    }

    pub fn user_config_path() -> PathBuf {
        Self::resolve_user_path(CONFIG_FILENAME)
    }

    pub fn locate_config_paths() -> Vec<PathBuf> {
        vec![Self::local_config_path(), Self::user_config_path()]
    }

    #[allow(dead_code)]
    pub fn locate_template_paths<T: AsRef<Path>>(p: T) -> Vec<PathBuf> {
        let suffix = Path::new("templates").join(p);
        vec![Self::resolve_local_path(&suffix), Self::resolve_user_path(&suffix)]
    }
}

#[allow(dead_code)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("load config failed"));
