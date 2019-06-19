use std::str;
use std::process;
use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_json as json;

use super::Error;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct CoreList {
    Platforms: Vec<Core>
}

/// A container for a line in the output produced by `arduino-cli core search ''`.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct Core {
    ID: String,
    Version: String,
    Name: String,
}

impl Core {

    pub fn id(&self) -> &str { &self.ID }

    pub fn version(&self) -> &str { &self.Version }

    pub fn name(&self) -> &str { &self.Name }
}

pub fn install_core(id: &str) -> Result<(), Error> {
    Command::new("arduino-cli")
        .args(&["core", "install", id])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()
        .map_err(|_| Error::CommandFailure)
        .and_then(|status| {
            if status.success() { Ok(()) } else { Err(Error::CommandFailure) }
        })
}

pub fn update_core_index() -> Result<(), Error> {
    Command::new("arduino-cli")
        .args(&["core", "update-index"])
        .output()
        .map(|_| ())
        .map_err(|_| Error::CommandFailure)
}

pub fn core_list_all() -> Result<Vec<Core>, Error> {
    // Asks the Arduino CLI for a list of all Arduino cores in JSON format.
    let command_stdout = Command::new("arduino-cli")
        .args(&["core", "search", "''", "--format", "json"])
        .output()
        .map(|output| output.stdout);

    if let Ok(stdout) = command_stdout {
        // The command line output has to be converted to a valid UTF-8 string before being able to
        // use it.
        str::from_utf8(&stdout)
            .map_err(|_| Error::CommandFailure)
            .and_then(cores_from_json)
    } else {
        Err(Error::CommandFailure)
    }
}

fn cores_from_json(core_json: &str) -> Result<Vec<Core>, Error> {
    json::from_str(core_json)
        .map(|core_list: CoreList| core_list.Platforms)
        .map_err(|_| Error::UnknownFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn core_1() -> Core {
        Core { ID: String::from("A"), Version: String::from("B"), Name: String::from("C") }
    }

    fn core_2() -> Core {
        Core { ID: String::from("1"), Version: String::from("2"), Name: String::from("3") }
    }

    /// A convenience function for creating the JSON-string, as would be printed by `arduino-cli
    /// core search '' --format json`, for a given list of cores.
    fn json_for_cores(cores: &Vec<Core>) -> String {
        let core_list = CoreList { Platforms: cores.clone() };
        String::from(json::json!(core_list).to_string())
    }

    #[test]
    fn no_cores() {
        let no_core_json = &json_for_cores(&vec![]);

        let result = cores_from_json(no_core_json).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn one_core() {
        let cores = vec![core_1()];
        let one_core_json = &json_for_cores(&cores);

        let result = cores_from_json(one_core_json).unwrap();

        assert_eq!(result, cores);
    }

    #[test]
    fn multiple_cores() {
        let cores = vec![core_1(), core_2()];
        let multi_core_json = &json_for_cores(&cores);

        let result = cores_from_json(multi_core_json).unwrap();

        assert_eq!(result, cores);
    }

    #[test]
    fn empty_json() {
        let err = cores_from_json("").unwrap_err();

        assert_eq!(err, Error::UnknownFormat);
    }

    #[test]
    fn malformed_json() {
        let malformed_json = r#"{"Platforms": [{"xyz": "0123"}]}"#;

        let err = cores_from_json(malformed_json).unwrap_err();

        assert_eq!(err, Error::UnknownFormat);
    }
}
