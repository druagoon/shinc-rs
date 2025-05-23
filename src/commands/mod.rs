mod build;
mod clean;
mod completions;
mod config;
mod dist;
mod homebrew;
mod man;
mod release;

use crate::prelude::*;

/// Bash CLI project manager using `argc`.
#[derive(clap::Subcommand, shinc_derive::CliCommand, Debug)]
pub enum Command {
    Build(build::BuildCmd),
    Clean(clean::CleanCmd),
    #[command(subcommand)]
    Config(config::ConfigCmd),
    Completions(completions::CompletionsCmd),
    Dist(dist::DistCmd),
    #[command(subcommand)]
    Homebrew(homebrew::HomebrewCmd),
    Man(man::ManCmd),
    Release(release::ReleaseCmd),
}
