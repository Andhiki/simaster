use std::process::ExitCode;

use clap::Parser;
use simaster_cli::cli::Cli;

fn main() -> ExitCode {
    match simaster_cli::run_with(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            error.exit_code()
        }
    }
}
