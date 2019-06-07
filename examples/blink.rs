// This is an example program demonstrating blinking an Arduino's digital pin 10.

use std::thread::sleep;
use std::time::Duration;

use arduino::Arduino;

fn main() {
    let mut arduino = match Arduino::new() {
        Ok(arduino) => arduino,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };

    let p10 = arduino::Pin::P10;

    arduino.set_pin_mode(&p10, &arduino::pin::Mode::Output);
    arduino.digital_write(&p10, &arduino::pin::State::High);

    sleep(Duration::from_secs(1));

    arduino.digital_write(&p10, &arduino::pin::State::Low);
}
