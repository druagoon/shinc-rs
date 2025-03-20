use std::ops::Deref;

use crate::config::Config;
use crate::prelude::*;

/// Show the active configuration.
#[derive(clap::Parser, Debug)]
pub struct ConfigShowCmd {
    #[command(flatten)]
    format: Format,
}

#[derive(clap::Args, Debug)]
struct Format {
    /// JSON output.
    #[arg(long, default_value_t)]
    json: bool,
    /// YAML output.
    #[arg(long, default_value_t)]
    yaml: bool,
}

impl ConfigShowCmd {
    fn render_json(&self, c: &Config) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(c)?)
    }

    fn render_yaml(&self, c: &Config) -> anyhow::Result<String> {
        Ok(serde_yaml::to_string(c)?)
    }

    fn render_toml(&self, c: &Config) -> anyhow::Result<String> {
        Ok(toml_edit::ser::to_string_pretty(c)?)
    }
}

impl CliCommand for ConfigShowCmd {
    fn run(&self) -> CliResult {
        let c = CONFIG.deref();
        let output = if self.format.json {
            self.render_json(c)?
        } else if self.format.yaml {
            self.render_yaml(c)?
        } else {
            self.render_toml(c)?
        };
        println!("{}", output.trim());

        Ok(())
    }
}
