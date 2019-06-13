//! This is an example program demonstrating interactions with the Arduino CLI.

use std::path::Path;
use arduinors::cli;

fn main() {
    let device_info = &cli::board_list_serial().unwrap()[0];

    println!("FQBN: {}", device_info.fqbn());
    println!("Port: {}", device_info.port());

    if device_info.has_unknown_core() {
        println!("The device's core is not installed.");
    }

    let sketch = Path::new("sketch-path");

    cli::compile(sketch, device_info).unwrap();
    cli::upload(sketch, device_info).unwrap();
}
