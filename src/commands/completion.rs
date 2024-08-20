use std::io;

use clap_complete::{Generator, Shell};

use crate::prelude::*;

/// Generate tab-completion scripts for terminal shell.
#[derive(clap::Parser, Debug)]
pub struct CompletionCmd {
    #[arg(value_enum)]
    shell: Shell,
}

impl CliCommand for CompletionCmd {
    fn run(&self) -> CliResult {
        let cmd = crate::cli::Cli::build();
        self.shell.generate(&cmd, &mut io::stdout());
        Ok(())
    }
}
