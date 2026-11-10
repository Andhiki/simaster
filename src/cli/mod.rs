use clap::{Parser, Subcommand};

use crate::error::Error;

pub mod auth;
pub mod completions;
pub mod doctor;
pub mod profile;

/// Unofficial terminal-first CLI for SIMASTER UGM.
#[derive(Debug, Parser)]
#[command(
    name = "simaster",
    version,
    about = "Unofficial terminal-first CLI for SIMASTER UGM",
    propagate_version = true
)]
pub struct Cli {
    /// Emit machine-readable JSON.
    #[arg(long, global = true)]
    pub json: bool,

    /// Enable debug logging to stderr.
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Command to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Authenticate with SIMASTER.
    Auth(auth::Auth),
    /// Inspect your SIMASTER profile.
    Profile(profile::Profile),
    /// Run offline diagnostics.
    Doctor(doctor::Doctor),
    /// Generate shell completions.
    Completions(completions::Completions),
}

/// Dispatch a parsed command.
pub fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Command::Auth(auth) => auth::run(auth),
        Command::Profile(profile) => profile::run(profile),
        Command::Doctor(doctor) => doctor::run(doctor, cli.json),
        Command::Completions(completions) => completions::run(completions),
    }
}
