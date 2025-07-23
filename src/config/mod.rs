use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

use crate::de::{deserialize_pathbuf, deserialize_pathbuf_option};
use crate::include_template;

/// The default configuration template.
pub const DEFAULT_CONFIG: &str = include_template!("config/default.toml");
/// The sub-directory where local or user configuration files are stored.
pub const CONFIG_SUBDIR: &str = concat!(".config/", clap::crate_name!());
/// The name of the configuration file.
pub const CONFIG_FILENAME: &str = "config.toml";

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    #[serde(skip)]
    root: PathBuf,
    project: Project,
    bin: Option<Vec<Bin>>,
    build: Option<Build>,
    dist: Dist,
    tools: Tools,
    release: Release,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Project {
    name: String,
    version: String,
    description: Option<String>,
    homepage: Option<url::Url>,
    repository: Option<url::Url>,
    license: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Bin {
    name: String,
    #[serde(default, deserialize_with = "deserialize_pathbuf")]
    path: PathBuf,
}

impl Bin {
    pub fn new<P: AsRef<Path>>(name: &str, path: P) -> Self {
        Self { name: name.to_string(), path: path.as_ref().to_path_buf() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Build {
    #[serde(default, deserialize_with = "deserialize_pathbuf_option")]
    target_dir: Option<PathBuf>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Dist {
    name: Option<String>,
    include_extra: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Tools {
    shfmt: Shfmt,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Shfmt {
    options: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Release {
    changelog: String,
}

#[allow(dead_code)]
impl Project {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn repository(&self) -> Option<&url::Url> {
        self.repository.as_ref()
    }
}

impl Config {
    pub fn new<P: AsRef<Path>>(root: P) -> anyhow::Result<Self> {
        let sources = vec![::config::File::from_str(DEFAULT_CONFIG, ::config::FileFormat::Toml)];
        let files: Vec<_> = [Self::locate_config_path(&root)]
            .iter()
            .rev()
            .filter(|&x| x.exists())
            .map(|x| ::config::File::from(x.to_owned()).required(false))
            .collect();
        if files.is_empty() {
            anyhow::bail!("No config file found");
        }
        let mut cfg = ::config::Config::builder()
            .add_source(sources)
            .add_source(files)
            .build()?
            .try_deserialize::<Self>()?;
        cfg.root = root.as_ref().to_path_buf();
        log::debug!("{cfg:#?}");
        Ok(cfg)
    }

    pub fn with_local() -> anyhow::Result<Self> {
        let root = std::env::current_dir().unwrap();
        Self::new(root)
    }

    pub fn locate_config_path<R: AsRef<Path>>(root: R) -> PathBuf {
        root.as_ref().join(CONFIG_SUBDIR).join(CONFIG_FILENAME)
    }

    pub fn path(&self) -> PathBuf {
        Self::locate_config_path(&self.root)
    }

    pub fn project(&self) -> &Project {
        &self.project
    }

    pub fn inferred_bins(&self) -> Vec<Bin> {
        let mut bins = vec![];
        let main_file = self.main_file();
        if main_file.exists() {
            bins.push(Bin::new(&self.project.name, main_file));
        }
        bins
    }

    pub fn bins(&self) -> Vec<Bin> {
        self.bin
            .clone()
            .unwrap_or(self.inferred_bins())
            .iter_mut()
            .map(|bin| {
                if bin.path.is_relative() {
                    bin.path = self.resolve_root_path(&bin.path);
                }
                bin.clone()
            })
            .collect()
    }

    pub fn resolve_root_path<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.root.join(p)
    }

    pub fn src_dir(&self) -> PathBuf {
        self.resolve_root_path("src")
    }

    pub fn resolve_src_path<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.src_dir().join(p)
    }

    pub fn main_file(&self) -> PathBuf {
        self.resolve_src_path("main.sh")
    }

    pub fn target_dir(&self) -> PathBuf {
        self.build
            .as_ref()
            .and_then(|b| b.target_dir.as_deref())
            .map_or_else(|| self.resolve_root_path("target"), |p| self.resolve_root_path(p))
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.target_dir().join("bin")
    }

    pub fn bin_file(&self, name: &str) -> PathBuf {
        self.bin_dir().join(name)
    }

    pub fn build_dir(&self) -> PathBuf {
        self.target_dir().join("build")
    }

    pub fn build_file(&self, name: &str) -> PathBuf {
        self.build_dir().join(name)
    }

    pub fn share_dir(&self) -> PathBuf {
        self.target_dir().join("share")
    }

    pub fn resolve_share_path<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.share_dir().join(p)
    }

    pub fn man_dir(&self) -> PathBuf {
        self.resolve_share_path("man")
    }

    pub fn man_file<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.man_dir().join(p)
    }

    pub fn comp_dir(&self) -> PathBuf {
        self.resolve_share_path("completions")
    }

    pub fn comp_file<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.comp_dir().join(p)
    }

    pub fn dist_dir(&self) -> PathBuf {
        self.target_dir().join("dist")
    }

    pub fn dist_file(&self, name: &str) -> PathBuf {
        self.dist_dir().join(name)
    }

    pub fn inferred_dist_name(&self) -> String {
        let bins = self.bins();
        self.dist_name()
            .unwrap_or_else(|| {
                if bins.len() != 1 {
                    self.project().name()
                } else {
                    bins.first().map(|b| b.name()).unwrap()
                }
            })
            .to_owned()
    }

    pub fn dist_name(&self) -> Option<&str> {
        self.dist.name.as_deref()
    }

    pub fn dist_include_extra_paths(&self) -> Vec<PathBuf> {
        self.dist.include_extra.iter().map(PathBuf::from).collect()
    }

    pub fn shfmt_options(&self) -> Vec<&str> {
        self.tools.shfmt.options.iter().map(String::as_str).collect()
    }

    pub fn changelog(&self) -> &str {
        &self.release.changelog
    }
}

#[allow(dead_code)]
pub static CONFIG: Lazy<Config> = Lazy::new(|| match Config::with_local() {
    Ok(cfg) => cfg,
    Err(e) => {
        eprintln!("load config failed: {e}");
        std::process::exit(1);
    }
});
