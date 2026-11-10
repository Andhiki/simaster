use clap::{Args, Subcommand, ValueEnum};

use crate::error::Error;

/// Authentication commands.
#[derive(Debug, Args)]
pub struct Auth {
    #[command(subcommand)]
    command: AuthCommand,
}

#[derive(Debug, Subcommand)]
enum AuthCommand {
    /// Sign in to SIMASTER.
    Login {
        /// Authentication method. `auto` prefers the web session flow.
        #[arg(long, value_enum, default_value = "auto")]
        method: LoginMethod,

        /// UGM username.
        #[arg(long)]
        username: Option<String>,

        /// Read the password from stdin instead of prompting.
        #[arg(long)]
        password_stdin: bool,
    },

    /// Show the current authentication state.
    Status,

    /// Remove stored credentials and sessions.
    Logout,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum LoginMethod {
    Auto,
    Sso,
    Web,
    Vnext,
}

/// Run an authentication command.
pub fn run(auth: Auth) -> Result<(), Error> {
    match auth.command {
        AuthCommand::Login {
            method,
            username,
            password_stdin,
        } => {
            tracing::debug!(
                ?method,
                username_provided = username.is_some(),
                password_stdin,
                "auth login requested"
            );
            Err(Error::NotImplemented("auth login"))
        }
        AuthCommand::Status => Err(Error::NotImplemented("auth status")),
        AuthCommand::Logout => Err(Error::NotImplemented("auth logout")),
    }
}
