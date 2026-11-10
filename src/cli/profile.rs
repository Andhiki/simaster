use clap::{Args, Subcommand};

use crate::error::Error;

/// Profile commands.
#[derive(Debug, Args)]
pub struct Profile {
    #[command(subcommand)]
    command: ProfileCommand,
}

#[derive(Debug, Subcommand)]
enum ProfileCommand {
    /// Show the authenticated student's profile.
    Show,
}

/// Run a profile command.
pub fn run(profile: Profile) -> Result<(), Error> {
    match profile.command {
        ProfileCommand::Show => Err(Error::NotImplemented("profile show")),
    }
}
