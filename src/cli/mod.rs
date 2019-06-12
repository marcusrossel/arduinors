//! This module provides an interface for accessing the Arduino CLI.

/// The errors that can occur as a result of interacting with the Arduino CLI.
#[derive(Debug)]
pub enum Error { CommandFailure, UnexpectedSyntax, NoDevice, MultipleDevices, InvalidSketchPath }

mod run;
mod query;

pub use run::*;
pub use query::*;
