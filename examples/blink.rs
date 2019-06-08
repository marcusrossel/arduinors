// This is an example program demonstrating blinking an Arduino's digital pin 10.

use std::thread::sleep;
use std::time::Duration;

use arduino::Arduino;

fn main() {
    let mut arduino = Arduino::new().unwrap();
    let pin_10 = arduino::Pin::new(10);

    arduino.set_pin_mode(&pin_10, &arduino::pin::Mode::Output);
    arduino.digital_write(&pin_10, &arduino::pin::State::High);

    sleep(Duration::from_secs(1));

    arduino.digital_write(&pin_10, &arduino::pin::State::Low);
}
