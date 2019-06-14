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
    serialBoards: Vec<Board>,
    networkBoards: Vec<Board>,
}

/// A container for a line in the output produced by `arduino-cli board list`.
///
/// A board may have placeholder values for the `name` and `fqbn` properties, if its core is not
/// installed. This can be checked via the `has_unknown_core` method.
///
/// You can get hold of board instances by calling `cli::board_list_serial`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct Board {
    name: String,
    fqbn: String,
    port: String,
    usbID: String,
}

impl Board {
    /// When a board is listed whose core has not been installed, it has this special name.
    const UNKNOWN_CORE_NAME: &'static str = "unknown";

    /// When a board is listed whose core has not been installed, has it this special FQBN.
    const UNKNOWN_CORE_FQBN: &'static str = "";

    /// Indicates whether the board's core is not installed (or *was* not when the info was
    /// captured).
    pub fn has_unknown_core(&self) -> bool {
        self.name == Board::UNKNOWN_CORE_NAME &&
        self.fqbn == Board::UNKNOWN_CORE_FQBN
    }

    pub fn board_name(&self) -> &str { &self.name }

    pub fn fqbn(&self) -> &str { &self.fqbn }

    pub fn port(&self) -> &str { &self.port }

    pub fn id(&self) -> &str { &self.usbID }
}

/// Calls `arduino-cli board list` and converts the resulting entries for serial boards into
/// `Board` instances.
/// Network boards are not returned, as they couldn't be connected to using this library's
/// `Arduino` type.
///
/// # Errors
/// * `CommandFailure`, if the `arduino-cli` command fails or produces non-UTF-8 output.
/// * `UnknownFormat`, if the call to the Arduino CLI produced an output in a different format
///   than expected.
pub fn board_list_serial() -> Result<Vec<Board>, Error> {
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
            .and_then(boards_from_list)
    } else {
        Err(Error::CommandFailure)
    }
}

/// Converts the serial boards in a given output from `arduino-cli board list --format json` into
/// board instances.
///
/// # Errors
/// * `UnknownFormat`, if deserialization is unsuccessful.
fn boards_from_list(board_list: &str) -> Result<Vec<Board>, Error> {
    // Deserialization is handeled automatically by `Board`'s derived conformance to serde's
    // `Deserialize`.
    json::from_str(board_list)
        .map(|board_list: BoardList| board_list.serialBoards)
        .map_err(|_| Error::UnknownFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A board with an installed core.
    fn some_board() -> Board {
        Board {
            name: String::from("A"), fqbn:  String::from("B"),
            port: String::from("C"), usbID: String::from("D"),
        }
    }

    /// A board without an installed core.
    fn coreless_board() -> Board {
        Board {
            name:  String::from(Board::UNKNOWN_CORE_NAME),
            fqbn:  String::from(Board::UNKNOWN_CORE_FQBN),
            port:  String::from("Y"),
            usbID: String::from("Z"),
        }
    }

    /// A convenience function for creating the JSON-string, as would be printed by `arduino-cli
    /// board list --format json`, for a given list of boards.
    fn list_for_boards(boards: &Vec<Board>) -> String {
        // Network boards are currently not taken into account.
        let board_list = BoardList {
            serialBoards: boards.clone(),
            networkBoards: vec![],
        };

        String::from(json::json!(board_list).to_string())
    }

    #[test]
    fn no_devices() {
        let no_board_json = &list_for_boards(&vec![]);

        let result = boards_from_list(no_board_json).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn one_device() {
        let device_infos = vec![some_board()];
        let one_board_json = &list_for_boards(&device_infos);

        let result = boards_from_list(one_board_json).unwrap();

        assert_eq!(result, device_infos);
    }

    #[test]
    fn multiple_devices() {
        let device_infos = vec![some_board(), coreless_board()];
        let multi_board_json = &list_for_boards(&device_infos);

        let result = boards_from_list(multi_board_json).unwrap();

        assert_eq!(result, device_infos);
    }

    #[test]
    fn empty_board_list() {
        let err = boards_from_list("").unwrap_err();

        assert_eq!(err, Error::UnknownFormat);
    }

    #[test]
    fn malformed_board_list() {
        let malformed_json = r#"{"serialBoards": [{"fqbn": "0123"}], "networkBoards": []}"#;

        let err = boards_from_list(malformed_json).unwrap_err();

        assert_eq!(err, Error::UnknownFormat);
    }
}
