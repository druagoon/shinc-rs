use std::fs;

use anyhow::Context;

use crate::config::Bin;
use crate::prelude::*;
use crate::utils::fs::ensure_dir_all;

/// Generate man pages.
#[derive(clap::Parser, Debug)]
pub struct ManCmd {}

impl CliCommand for ManCmd {
    fn run(&self) -> CliResult {
        for bin in &CONFIG.bins() {
            gen_man(bin)?;
        }

        Ok(())
    }
}

fn gen_man(bin: &Bin) -> anyhow::Result<()> {
    let bin_name = bin.name();
    let target = CONFIG.bin_file(bin_name);
    if !target.is_file() {
        anyhow::bail!("bin file not found: {}", target.display());
    }

    ensure_dir_all(CONFIG.man_dir())?;
    let source = fs::read_to_string(target)?;
    let pages = argc::mangen(&source, bin_name)?;
    for (filename, page) in pages {
        let man_file = CONFIG.man_file(&filename);
        fs::write(&man_file, page)
            .with_context(|| format!("failed to write '{}'", man_file.display()))?;
        println!("{}", man_file.display());
    }
    Ok(())
}
