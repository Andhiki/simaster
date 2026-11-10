use std::io::Write;

use serde::Serialize;

use crate::error::Error;

/// Write a JSON document to stdout followed by a newline.
pub fn print_json<T: Serialize>(value: &T) -> Result<(), Error> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    serde_json::to_writer_pretty(&mut handle, value)?;
    writeln!(handle)?;
    Ok(())
}
