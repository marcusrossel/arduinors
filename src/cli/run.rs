use std::process;
use std::path::Path;

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
    let compilation_result = process::Command::new("arduino-cli")
        .args(&["compile", "--fqbn", &fqbn[..], path])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status();

    match compilation_result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(Error::CommandFailure),
    }
}

/// Uploads a **compiled** sketch onto the currently connected Arduino.
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
    let compilation_result = process::Command::new("arduino-cli")
        .args(&["upload", "--port", &port[..], "--fqbn", &fqbn[..], path])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status();

    match compilation_result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(Error::CommandFailure),
    }
}

/// Converts a given sketch-path to its string representation, while validating the path in the
/// process.
fn sketch_to_str(sketch: &Path) -> Result<&str, Error> {
    // An Arduino sketch must be a directory with a valid UTF-8 name.
    match sketch.to_str() {
        Some(path) if sketch.is_dir() => Ok(path),
        _ => Err(Error::InvalidSketchPath),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_sketch_path() {
        let invalid_path = Path::new(":X/\\:y/z");

        let result = sketch_to_str(invalid_path);
        let err = result.unwrap_err();

        assert_eq!(err, Error::InvalidSketchPath);
    }
}
