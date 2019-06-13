use crate::cli::DeviceInfo;

use crate::pin;
use crate::pin::Pin;

/// A handle on an Arduino, for communicating with it via the Firmata protocol.
pub struct Arduino(firmata::Board);

impl Arduino {

    /// Creates an Arduino with the port contained in given device info.
    pub fn from(device_info: &DeviceInfo) -> Arduino {
        Arduino(firmata::Board::new(device_info.port()))
    }

    /// Writes the given pin state to the given pin of the Arduino.
    pub fn digital_write(&mut self, pin: Pin, state: pin::State) {
        self.0.digital_write(pin.value(), state as i32);
    }

    /// Writes the given pin level to the given pin of the Arduino.
    pub fn analog_write(&mut self, pin: Pin, level: pin::Level) {
        self.0.analog_write(pin.value(), level as i32);
    }

    /// Set the given pin of the Arduino to the given mode.
    pub fn set_pin_mode(&mut self, pin: Pin, mode: pin::Mode) {
        self.0.set_pin_mode(pin.value(), mode as u8);
    }
}
