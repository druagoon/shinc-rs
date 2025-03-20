use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context;
use clap::ValueEnum;
use clap_complete::aot::Shell;
use clap_complete::Generator;

use crate::config::Bin;
use crate::prelude::*;
use crate::utils::formatter::identifier;
use crate::utils::fs::create_file;
use crate::utils::tips;

/// Generate shell completion scripts.
#[derive(clap::Parser, Debug)]
pub struct CompletionsCmd {}

impl CliCommand for CompletionsCmd {
    fn run(&self) -> CliResult {
        for bin in &CONFIG.bins() {
            for shell in Shell::value_variants() {
                gen_completions(bin, shell)?;
            }
        }

        Ok(())
    }
}

fn starts_with_compdef(s: &str) -> bool {
    s.lines().next().is_some_and(|line| line.starts_with("#compdef "))
}

fn gen_completions(bin: &Bin, shell: &Shell) -> anyhow::Result<()> {
    let bin_name = bin.name();
    let argc_shell = argc::Shell::from_str(&shell.to_string())?;
    let commands = vec!["argc".to_string(), bin_name.to_string()];
    let mut content = argc::generate_completions(argc_shell, &commands);
    if shell == &Shell::Zsh && !starts_with_compdef(&content) {
        content = format!("#compdef {}\n\n{content}", commands.join(" "));
    };
    let comp_file = CONFIG.comp_file(
        Path::new(shell.to_possible_value().unwrap().get_name()).join(shell.file_name(bin_name)),
    );
    tips::h1(&format!(
        "Generating {} completions for {}",
        identifier(&shell.to_string()),
        identifier(bin_name)
    ));
    create_file(&comp_file)?
        .write_all(content.as_bytes())
        .with_context(|| format!("failed to write script to '{}'", comp_file.display()))?;
    println!("{}", comp_file.display());
    Ok(())
}
