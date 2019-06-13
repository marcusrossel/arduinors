use std::str;
use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_json as json;

use super::Error;

/// A wrapper for the result of calling `arduino-cli board list --format json`, in order to take
/// advantage of serde's derived JSON (de)serialization.
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BoardList {
    serialBoards: Vec<DeviceInfo>,
    networkBoards: Vec<DeviceInfo>,
}

/// A container for a line in the output produced by `arduino-cli board list`.
///
/// A device info may have placeholder values for the `name` and `fqbn` properties, if the
/// associated board's core is not installed. This can be checked via the `has_unknown_core`
/// method.
///
/// You can get hold of device info instances by calling `cli::board_list_serial`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct DeviceInfo {
    name: String,
    fqbn: String,
    port: String,
    usbID: String,
}

impl DeviceInfo {
    /// When a device is listed whose core has not been installed, it has this special name.
    const UNKNOWN_BOARD_NAME: &'static str = "unknown";

    /// When a device is listed whose core has not been installed, has it this special FQBN.
    const UNKNOWN_BOARD_FQBN: &'static str = "";

    /// Indicates whether the device info belongs to a board whose core is not installed (or *was*
    /// not when the device info was captured).
    pub fn has_unknown_core(&self) -> bool {
        self.name == DeviceInfo::UNKNOWN_BOARD_NAME &&
        self.fqbn == DeviceInfo::UNKNOWN_BOARD_FQBN
    }

    pub fn board_name(&self) -> &str { &self.name }

    pub fn fqbn(&self) -> &str { &self.fqbn }

    pub fn port(&self) -> &str { &self.port }

    pub fn id(&self) -> &str { &self.usbID }
}

/// Calls `arduino-cli board list` and converts the resulting entries for serial boards into
/// `DeviceInfo` instances.
/// Network boards are not returned, as they couldn't be connected to using this library's
/// `Arduino` type.
///
/// # Errors
/// * `CommandFailure`, if the `arduino-cli` command fails or produces non-UTF-8 output.
/// * `UnknownFormat`, if the call to the Arduino CLI produced an output in a different format
///   than expected.
pub fn board_list_serial() -> Result<Vec<DeviceInfo>, Error> {
    // Asks the Arduino CLI for a list of connected Arduinos in JSON format.
    let command_stdout = Command::new("arduino-cli")
        .args(&["board", "list", "--format", "json"])
        .output()
        .map(|output| output.stdout);

    if let Ok(stdout) = command_stdout {
        // The command line output has to be converted to a valid UTF-8 string before being able to
        // use it.
        str::from_utf8(&stdout)
            .map_err(|_| Error::CommandFailure)
            .and_then(device_infos_from_board_list)
    } else {
        Err(Error::CommandFailure)
    }
}

/// Converts a given output from `arduino-cli board list --format json` into a list of device infos
/// of the serial boards.
///
/// # Errors
/// * `UnknownFormat`, if deserialization is unsuccessful.
fn device_infos_from_board_list(board_list: &str) -> Result<Vec<DeviceInfo>, Error> {
    // Deserialization is handeled automatically by `DeviceInfo`'s derived conformance to serde's
    // `Deserialize`.
    json::from_str(board_list)
        .map(|board_list: BoardList| board_list.serialBoards)
        .map_err(|_| Error::UnknownFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The device info for a device with an installed core.
    fn some_device_info() -> DeviceInfo {
        DeviceInfo {
            name: String::from("A"), fqbn:  String::from("B"),
            port: String::from("C"), usbID: String::from("D"),
        }
    }

    /// The device info for a device without an installed core.
    fn unknown_device_info() -> DeviceInfo {
        DeviceInfo {
            name: String::from(DeviceInfo::UNKNOWN_BOARD_NAME),
            fqbn: String::from(DeviceInfo::UNKNOWN_BOARD_FQBN),
            port: String::from("Y"),
            usbID: String::from("Z"),
        }
    }

    /// A convenience function for creating the JSON-string, as would be printed by `arduino-cli
    /// board list --format json`, for a given list of device infos.
    fn board_list_for_device_infos(device_infos: &Vec<DeviceInfo>) -> String {
        // Network boards are currently not taken into account.
        let board_list = BoardList {
            serialBoards: device_infos.clone(),
            networkBoards: vec![],
        };

        String::from(json::json!(board_list).to_string())
    }

    #[test]
    fn no_devices() {
        let no_board_json = &board_list_for_device_infos(&vec![]);

        let result = device_infos_from_board_list(no_board_json).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn one_device() {
        let device_infos = vec![some_device_info()];
        let one_board_json = &board_list_for_device_infos(&device_infos);

        let result = device_infos_from_board_list(one_board_json).unwrap();

        assert_eq!(result, device_infos);
    }

    #[test]
    fn multiple_devices() {
        let device_infos = vec![some_device_info(), unknown_device_info()];
        let multi_board_json = &board_list_for_device_infos(&device_infos);

        let result = device_infos_from_board_list(multi_board_json).unwrap();

        assert_eq!(result, device_infos);
    }

    #[test]
    fn empty_board_list() {
        let err = device_infos_from_board_list("").unwrap_err();
        
        assert_eq!(err, Error::UnknownFormat);
    }

    #[test]
    fn malformed_board_list() {
        let malformed_json = r#"{"serialBoards": [{"fqbn": "0123"}], "networkBoards": []}"#;

        let err = device_infos_from_board_list(malformed_json).unwrap_err();

        assert_eq!(err, Error::UnknownFormat);
    }
}
