use std::process;
use std::path::Path;
use std::fs;

use super::DeviceInfo;
use super::Error;

/// Compiles a sketch at a given path, for the device with the given info.
/// The given path should point to the sketch **directory**, not **file**.
///
/// # Errors
/// * `CommandFailure`, if the `arduino-cli` command fails or an error occurs during compilation.
///   This will definitely occur if the given device info in unknown.
/// * `InvalidSketchPath`, if the sketch does not have the format required for Arduino sketches.
pub fn compile(sketch: &Path, device_info: &DeviceInfo) -> Result<(), Error> {
    // Command failure would occur if this device info was used.
    if device_info.has_unknown_core() { return Err(Error::CommandFailure); }

    let path = sketch_to_string(sketch)?;

    // Asks the Arduino CLI to compile the given sketch.
    let compilation_result = process::Command::new("arduino-cli")
        .args(&["compile", "--fqbn", device_info.fqbn(), &path])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status();

    match compilation_result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(Error::CommandFailure),
    }
}

/// Uploads a **compiled** sketch onto Arduino with the given device info.
/// The given path should point to the sketch **directory**, not **file**.
///
/// # Errors
/// * `CommandFailure`, if the `arduino-cli` command fails or an error occurs during uploading.
///   This will definitely occur if the given device info in unknown, or the Arduino is not
///   connected.
/// * `InvalidSketchPath`, if the sketch does not have the format required for Arduino sketches.
pub fn upload(sketch: &Path, device_info: &DeviceInfo) -> Result<(), Error> {
    // Command failure would occur if this device info was used.
    if device_info.has_unknown_core() { return Err(Error::CommandFailure); }

    let path = sketch_to_string(sketch)?;

    // Asks the Arduino CLI to upload the given compiled sketch.
    let compilation_result = process::Command::new("arduino-cli")
        .args(&["upload", "--port", device_info.port(), "--fqbn", device_info.fqbn(), &path])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status();

    match compilation_result {
        Ok(status) if status.success() => Ok(()),
        _ => Err(Error::CommandFailure),
    }
}

/// Converts a given sketch-path to its canonical string representation, while validating it in the
/// process.
fn sketch_to_string(sketch: &Path) -> Result<String, Error> {
    // An Arduino sketch must be a directory with a valid UTF-8 name, that contains a .ino-file of
    // the same name.
    if let Ok(canonical_path) = sketch.canonicalize() {
        if canonical_path.is_dir() {
            if let Ok(sketch_files) = fs::read_dir(&canonical_path) {
                if let Some(sketch_name) = canonical_path.file_name() {
                    let sketch_files_paths: Vec<_> = sketch_files
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.file_name())
                        .collect();

                    if let Some(sketch_name) = sketch_name.to_str() {
                        let sketch_file = format!("{}.ino", sketch_name);

                        if sketch_files_paths.contains(&std::ffi::OsString::from(sketch_file)) {
                            if let Some(sketch_path) = canonical_path.to_str() {
                                return Ok(String::from(sketch_path));
                            }
                        }
                    }
                }
            }
        }
    }

    Err(Error::InvalidSketchPath)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_sketch_path_to_str() {
        // A path that should be invalid on all systems.
        let invalid_path = Path::new(":X/\\:y/z");

        let result = sketch_to_string(invalid_path);
        let err = result.unwrap_err();

        assert_eq!(err, Error::InvalidSketchPath);
    }
}
