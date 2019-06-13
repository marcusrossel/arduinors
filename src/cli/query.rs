use std::str;
use std::process::Command;
use regex::Regex;

use super::Error;

/// Items that the Arduino CLI can be queried for.
#[derive(Clone, Copy)]
pub enum Query {
    Fqbn = 0,
    Port = 1,
    Id = 2,
    BoardName = 3,
}

/// Extracts the item associated with a given query by calling the Arduino CLI - or more
/// specifically `arduino-cli board list`.
///
/// # Errors
/// * `CommandFailure`, if the `arduino-cli` command fails or produces non-UTF-8 output.
/// * `NoDevice`, if no Arduino is connected to the computer during the call.
/// * `MultipleDevices`, if more than one Arduino is connected to the computer during the call.
/// * `MissingCore`, if a single Arduino device was found, but its core is not installed.
/// * `UnexpectedSyntax`, if the call to the Arduino CLI produced an output in a different format
///   than expected.
pub fn query(query: Query) -> Result<String, Error> {
    // Asks the Arduino CLI for connected Arduinos.
    let output = Command::new("arduino-cli")
        .args(&["board", "list"])
        .output();

    // Makes sure the call to the Arduino CLI even worked.
    match output {
        Ok(output) => {
            // Turns the result of the previous call into a string.
            let board_list = match str::from_utf8(&output.stdout) {
                Ok(board_list) => board_list,
                Err(_) => return Err(Error::CommandFailure)
            };

            query_from_board_list(query, board_list)
        }

        Err(_) => Err(Error::CommandFailure)
    }
}

/// Extracts a given query item from a given output of `arduino-cli board list`.
///
/// # Errors
/// This function calls `query_from_board_entry`, and will pass along any errors produced by it.
/// * `NoDevice`, if no Arduino is connected to the computer during the call.
/// * `MultipleDevices`, if more than one Arduino is connected to the computer during the call.
fn query_from_board_list(query: Query, board_list: &str) -> Result<String, Error> {
    let emtpy_line = Regex::new(r"^\s*$").unwrap();

    // A container in which the single board entry will be placed.
    let mut board_entry: Option<&str> = None;

    // Fills the board entry container with the single board's entry, and returns an error if
    // multiple entries were found.
    // The first line in the board list is the header and is therefore skipped.
    for line in board_list.lines().skip(1) {
        if !emtpy_line.is_match(line) {
            if board_entry.is_none() {
                board_entry = Some(line);
            } else {
                return Err(Error::MultipleDevices);
            }
        }
    }

    // If the board entry container is still empty at this point, no Arduino was found.
    if let Some(board_entry) = board_entry {
        query_from_board_entry(query, board_entry)
    } else {
        Err(Error::NoDevice)
    }
}

/// Extracts a given query item from a given board entry of the output of `arduino-cli board
/// list`.
///
/// # Errors
/// The entry is expected to have the format:
/// `<fqbn>\t<port>\t<id>\t<board name>`
///
/// * `UnexpectedSyntax`, if the board entry does not have the expected format.
/// * `MissingCore`, if the board entry's FQBN-field was empty (implying that the board's core
///    is not installed).
fn query_from_board_entry(query: Query, board_entry: &str) -> Result<String, Error> {
    const REQUIRED_FIELD_COUNT: usize = 4;
    let query_column = query as usize;

    // Fields in a board entry are seperated by tabs.
    let fields: Vec<_> = board_entry.split('\t').collect();

    if fields.len() != REQUIRED_FIELD_COUNT { return Err(Error::UnexpectedSyntax); }

    // The fields container will definitely contain a value for any query, as the field count is
    // valid.

    // If the FQBN field is only whitespace, the Arduino's core is not installed.
    if fields[Query::Fqbn as usize].trim().is_empty() {
        Err(Error::MissingCore)
    } else {
        Ok(String::from(fields[query_column]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEVICE_LIST_HEADER: &'static str = "FQBN\tPort\tID\tBoard Name";
    const SOME_QUERY_ITEM: &'static str = "Query::Item/1'\\\"_";
    const SOME_QUERY: Query = Query::Fqbn;

    #[test]
    fn no_devices() {
        let no_device_list = format!("{}\n", DEVICE_LIST_HEADER);

        let result = query_from_board_list(SOME_QUERY, &no_device_list);
        let err = result.unwrap_err();

        assert_eq!(err, Error::NoDevice);
    }

    #[test]
    fn multiple_devices() {
        let multi_device_list = format!("{}\n1\t2\t3\t4\nA\tB\tC\tD\n", DEVICE_LIST_HEADER);

        let result = query_from_board_list(SOME_QUERY, &multi_device_list);
        let err = result.unwrap_err();

        assert_eq!(err, Error::MultipleDevices);
    }

    #[test]
    fn unexpected_syntax() {
        const INVALID_ENTRY: &'static str = "1\t2\t3\t4\t5\n";

        let result = query_from_board_entry(SOME_QUERY, INVALID_ENTRY);
        let err = result.unwrap_err();

        assert_eq!(err, Error::UnexpectedSyntax);
    }

    #[test]
    fn valid_list() {
        let valid_list = format!("{}\n\t\n1\t2\t3\t4\n\t\n", DEVICE_LIST_HEADER);

        let result = query_from_board_list(SOME_QUERY, &valid_list);

        assert!(result.is_ok());
    }

    #[test]
    fn valid_fqbn_query() {
        let board_list = format!("{}\n{}\t2\t3\t4\n\t\n", DEVICE_LIST_HEADER, SOME_QUERY_ITEM);
        let query = Query::Fqbn;

        let result = query_from_board_list(query, &board_list);
        let query_item = result.unwrap();

        assert_eq!(query_item, SOME_QUERY_ITEM);
    }

    #[test]
    fn valid_port_query() {
        let board_list = format!("{}\n1\t{}\t3\t4\n\t\n", DEVICE_LIST_HEADER, SOME_QUERY_ITEM);
        let query = Query::Port;

        let result = query_from_board_list(query, &board_list);
        let query_item = result.unwrap();

        assert_eq!(query_item, SOME_QUERY_ITEM);
    }
}
