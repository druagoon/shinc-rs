mod completion;
mod config;

use crate::prelude::*;

#[derive(clap::Subcommand, shinc_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Config(config::ConfigCmd),
    Completion(completion::CompletionCmd),
}
