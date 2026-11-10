use std::io::stdout;

use clap::{Args, CommandFactory};
use clap_complete::Shell;

use crate::error::Error;

/// Generate shell completions.
#[derive(Debug, Args)]
pub struct Completions {
    /// Shell to generate completions for.
    #[arg(value_enum)]
    shell: Shell,
}

/// Write completions for the requested shell to stdout.
pub fn run(completions: Completions) -> Result<(), Error> {
    let mut command = crate::cli::Cli::command();
    clap_complete::generate(completions.shell, &mut command, "simaster", &mut stdout());
    Ok(())
}
