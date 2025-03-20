mod formula;

use crate::prelude::*;

/// Homebrew-related commands.
#[derive(clap::Subcommand, shinc_derive::CliCommand, Debug)]
pub enum HomebrewCmd {
    Formula(formula::HomebrewFormulaCmd),
}
