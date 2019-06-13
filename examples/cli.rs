//! This is an example program demonstrating interactions with the Arduino CLI.

use std::path::Path;
use arduinors::cli;

fn main() {
    let fqbn = cli::query(cli::Query::Fqbn).unwrap();
    println!("FQBN: {}", fqbn);

    let port = cli::query(cli::Query::Port).unwrap();
    println!("Port: {}", port);

    let sketch = Path::new("sketch-path");

    cli::compile(sketch).unwrap();
    cli::upload(sketch).unwrap();
}
