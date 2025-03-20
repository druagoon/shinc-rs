use std::io;

use clap::{CommandFactory, Parser};
use clap_complete::aot::{Generator, Shell};

use crate::commands::Command;
use crate::error::CliResult;

pub trait CliCommand {
    fn run(&self) -> CliResult;
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(bin_name = clap::crate_name!())]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
    #[arg(long, value_name = "SHELL", value_enum)]
    generate_shell_completions: Option<Shell>,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Cli {
    pub fn exec() {
        let cli = Self::parse();
        if let Err(e) = cli.run() {
            eprintln!("{:?}", e);
            ::std::process::exit(1);
        }
    }

    /// See also [`clap::Command::build`]
    ///
    /// can be used for completions.
    #[allow(dead_code)]
    pub fn build() -> clap::Command {
        let mut cmd = Self::command();
        cmd.build();
        cmd
    }

    fn init(&self) {
        self.init_logging()
    }

    /// Initialize logging system.
    fn init_logging(&self) {
        let level = self.verbose.log_level_filter();
        env_logger::Builder::new().filter_level(level).init();
        log::debug!("initialize logging system at log level: {}", level);
    }
}

impl CliCommand for Cli {
    fn run(&self) -> CliResult {
        self.init();
        match &self.command {
            Some(cmd) => cmd.run(),
            None => {
                if let Some(shell) = self.generate_shell_completions {
                    let cmd = crate::cli::Cli::build();
                    shell.generate(&cmd, &mut io::stdout());
                } else {
                    Self::command().print_long_help()?;
                }
                Ok(())
            }
        }
    }
}
