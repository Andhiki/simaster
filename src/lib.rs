//! Offline walking skeleton for the `simaster` CLI.
//!
//! Authentication and SIMASTER API access are planned for later milestones.
//! The available commands are `doctor` and `completions`.

pub mod cli;
pub mod config;
pub mod error;
pub mod output;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::error::Error;

/// Parse arguments, initialize logging, and dispatch the selected command.
pub fn run() -> Result<(), Error> {
    let cli = Cli::parse();
    init_tracing(cli.verbose);
    cli::run(cli)
}

fn init_tracing(verbose: bool) {
    let default_filter = if verbose { "debug" } else { "warn" };
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();
}
