use clap::{CommandFactory, Parser};

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
                Self::command().print_long_help()?;
                Ok(())
            }
        }
    }
}
