use std::process::ExitStatus;
use std::process::Command;
use std::path::Path;
use std::io;

use super::Error;
use super::query::*;

/// Compiles a sketch at a given path, for the currently connected Arduino.
/// The given path should point to the sketch **directory**, not **file**.
///
/// # Errors
/// This function calls `arduino::cli::query`, and will pass along any errors produced by it.
/// * `CommandFailure`, if the `arduino-cli` command fails or an error occurs during compilation.
/// * `InvalidSketchPath`, if the sketch does not have the format required for Arduino sketches.
pub fn compile(sketch: &Path) -> Result<(), Error> {
    // Compilation requires the sketch path and FQBN as parameters.
    let path = sketch_to_str(sketch)?;
    let fqbn = query(Query::Fqbn)?;

    // Asks the Arduino CLI to compile the given sketch.
    let compilation_result = Command::new("arduino-cli")
        .args(&["compile", "--fqbn", &fqbn[..], path])
        .status();

    status_to_result(&compilation_result)
}

/// Uploads a compiled sketch onto the currently connected Arduino.
/// The given path should point to the sketch **directory**, not **file**.
///
/// # Errors
/// This function calls `arduino::cli::query`, and will pass along any errors produced by it.
/// * `CommandFailure`, if the `arduino-cli` command fails or an error occurs during uploading.
/// * `InvalidSketchPath`, if the sketch does not have the format required for Arduino sketches.
pub fn upload(sketch: &Path) -> Result<(), Error> {
    // Uploading requires the sketch path, Arduino's FQBN and port as parameters.
    let path = sketch_to_str(sketch)?;
    let fqbn = query(Query::Fqbn)?;
    let port = query(Query::Port)?;

    // Asks the Arduino CLI to upload the given compiled sketch.
    let compilation_result = Command::new("arduino-cli")
        .args(&["upload", "--port", &port[..], "--fqbn", &fqbn[..], path])
        .status();

    status_to_result(&compilation_result)
}

// Converts a given sketch-path to its string representation, validating the validity of the
// path in the process.
fn sketch_to_str(sketch: &Path) -> Result<&str, Error> {
    // An Arduino sketch must be a directory with a valid UTF-8 name.
    if !sketch.is_dir() { return Err(Error::InvalidSketchPath); }
    match sketch.to_str() {
        Some(path) => Ok(path),
        None => return Err(Error::InvalidSketchPath)
    }
}

// Returns on failure if the given status indicates that command execution itself failed or the
// command returned on failure.
fn status_to_result(status: &Result<ExitStatus, io::Error>) -> Result<(), Error> {
    match status {
        Ok(status) => if !status.success() { return Err(Error::CommandFailure) },
        Err(_) => return Err(Error::CommandFailure)
    }

    Ok(())
}
