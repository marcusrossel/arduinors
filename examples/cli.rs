//! This is an example program demonstrating interactions with the Arduino CLI.

use std::path::Path;
use arduinors::cli;

fn main() -> Result<(), cli::Error> {
    let board = &cli::board_list_serial().unwrap()[0];

    println!("FQBN: {}", board.fqbn());
    println!("Port: {}", board.port());

    if board.has_unknown_core() {
        println!("The device's core is not installed.");
    }

    let sketch = Path::new("sketch-path");

    cli::compile(sketch, board)?;
    cli::upload(sketch, board)?;

    Ok(())
}
