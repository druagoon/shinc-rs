use crate::config::Config;
use crate::prelude::*;

/// List the configuration files.
#[derive(clap::Parser, Debug)]
pub struct ConfigListCmd {
    /// Only list the existing configuration files.
    #[arg(long, default_value_t)]
    exists: bool,
    /// Print configuration files content.
    #[arg(long, default_value_t)]
    with_content: bool,
}

impl CliCommand for ConfigListCmd {
    fn run(&self) -> CliResult {
        let mut i = 0;
        let files = Config::locate_config_paths();
        for f in files {
            if !self.exists || f.exists() {
                i += 1;
                println!("{}: {}", i, f.display());
            }
            if self.with_content && f.exists() {
                let content = std::fs::read_to_string(f)?;
                println!("{}\n", content.trim());
            }
        }
        Ok(())
    }
}
