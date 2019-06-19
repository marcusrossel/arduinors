//! This is an example program demonstrating blinking an Arduino's digital pin 10.

use std::thread::sleep;
use std::time::Duration;

use arduinors as arduino;
use arduino::Arduino;

fn main() -> Result<(), arduino::Error> {
    let board = &arduino::cli::board_list_serial().unwrap()[0];

    let mut arduino = Arduino::from(board);

    arduino.set_pin_mode(10, arduino::PinMode::DigitalOutput)?;
    arduino.write(10, 1)?;

    sleep(Duration::from_secs(1));

    arduino.write(10, 0)?;

    Ok(())
}
