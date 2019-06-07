// An interface for accessing the Arduino CLI.
// This module's functions expect that the Arduino CLI is installed and accessible using the
// "arduino-cli" command - otherwise an error will occur.

use std::str;
use std::process::Command;
use regex::Regex;

// A list of items that the Arduino CLI can be queried for.
pub enum Query { Fqbn, Port }

// The errors that can occur as a result of querying the Arduino CLI.
#[derive(Debug)]
pub enum Error { CommandFailure, UnexpectedSyntax, NoDevice, MultipleDevices }

// Extracts the item associated with a given query by calling the Arduino CLI. If this process
// fails an error is returned.
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

// Extracts a given query item from a given output of "arduino-cli board list". If that is not
// possible an error is returned.
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

// Extracts a given query item from a given board entry of the output of "arduino-cli board
// list".
// The entry is expected to have the format:
// <fqbn> <port> <id> <board name>
// If it does not, an error is returned.
fn query_from_board_entry(query: Query, board_entry: &str) -> Result<String, Error> {
    // The required field count is 4, as the expected format of a board list enty above shows.
    // The field count might be higher though, as a field may contain white space. This will
    // not affect the current query items though, as they will not contain whitespace (?).
    const REQUIRED_FIELD_COUNT: u8 = 4;
    let mut field_count = 0;

    let query_column = match query { Query::Fqbn => 1, Query::Port => 2, };
    let mut query_item: Option<&str> = None;

    // Iterates over the fields in the entry, on the one hand to extract the query item, and on
    // the other hand to count the number of fields.
    for field in board_entry.split_whitespace() {
        field_count += 1;
        if field_count == query_column { query_item = Some(field); }
    }

    // The query item container will definitely contain a value, if the field count has reached
    // the required field count.
    if field_count >= REQUIRED_FIELD_COUNT {
        Ok(String::from(query_item.unwrap()))
    } else {
        Err(Error::UnexpectedSyntax)
    }
}
