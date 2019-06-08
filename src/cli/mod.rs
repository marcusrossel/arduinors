// An interface for accessing the Arduino CLI.
// This module's functions expect that the Arduino CLI is installed and accessible using the
// "arduino-cli" command - otherwise an error will occur.

// The errors that can occur as a result of querying the Arduino CLI.
#[derive(Debug)]
pub enum Error { CommandFailure, UnexpectedSyntax, NoDevice, MultipleDevices, InvalidSketchPath }

mod run;
mod query;

pub use run::*;
pub use query::*;
