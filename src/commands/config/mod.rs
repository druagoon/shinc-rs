mod generate;
mod list;
mod show;

use crate::prelude::*;

/// Manage local and global configuration.
#[derive(clap::Subcommand, shinc_derive::CliCommand, Debug)]
pub enum ConfigCmd {
    Generate(generate::ConfigGenerateCmd),
    List(list::ConfigListCmd),
    Show(show::ConfigShowCmd),
}
