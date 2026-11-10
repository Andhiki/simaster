use std::process::ExitCode;

fn main() -> ExitCode {
    match simaster_cli::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            error.exit_code()
        }
    }
}
