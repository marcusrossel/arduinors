//! # Arduinors
//! This library provides an interface for working with Arduino-related tasks.
//! It provides a Firmata-based interface for manipulating Arduino boards, as well as an interface
//! for working with the Arduino CLI.
//!
//! # Expectations
//! * the Arduino CLI is installed and accessible using the `arduino-cli` command.
//! * there is exactly one Arduino connected to the computer.
//!
//! Not meeting these expectations will result in errors for almost all function/method calls.

mod arduino;
pub use arduino::*;

pub mod cli;
pub use cli::Board;
