use std::fs;
use std::io::Write;

use crate::config::DEFAULT_CONFIG;
use crate::prelude::*;
use crate::utils::fs::create_file;
use crate::utils::path::add_extension;
use crate::utils::tips;

/// Generate the configuration files.
#[derive(clap::Parser, Debug)]
pub struct ConfigGenerateCmd {
    /// Force overwrite even if the configuration file already exists.
    #[arg(long, default_value_t)]
    force: bool,
}

impl CliCommand for ConfigGenerateCmd {
    fn run(&self) -> CliResult {
        let config_path = CONFIG.path();
        if self.force || !config_path.exists() {
            if config_path.exists() {
                let backup_path = add_extension(&config_path, "bak")?;
                fs::copy(&config_path, &backup_path)?;
                tips::h1("Backing up");
                println!("{} -> {}", config_path.display(), backup_path.display());
            }
            create_file(&config_path)?.write_all(DEFAULT_CONFIG.as_bytes())?;
            tips::h1("Generating");
            println!("{}", config_path.display());
        } else {
            tips::error(&format!(
                "Config file already exists '{}', use --force to overwrite.",
                config_path.display()
            ));
        }

        Ok(())
    }
}
