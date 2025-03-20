mod cli;
mod commands;
mod config;
mod consts;
mod de;
mod error;
mod macros;
mod prelude;
mod utils;

fn main() {
    self::cli::Cli::exec();
}
