use crate::cli;
pub use cli::Error;

pub use crate::pin;
pub use crate::pin::Pin;

/// A handle on an Arduino, for communicating with it via the Firmata protocol.
pub struct Arduino(firmata::Board);

impl Arduino {

    /// Creates an Arduino object from the single currently connected Arduino device.
    ///
    /// # Errors
    /// This function calls `arduino::cli::query`, and will pass along any errors produced by it.
    pub fn new() -> Result<Arduino, Error> {
        let port = cli::query(cli::Query::Port)?;
        let arduino = Arduino(firmata::Board::new(&port[..]));

        Ok(arduino)
    }

    /// Writes the given pin state to the given pin of the Arduino.
    pub fn digital_write(&mut self, pin: Pin, state: pin::State) {
        self.0.digital_write(pin.value(), state.value());
    }

    /// Writes the given pin level to the given pin of the Arduino.
    pub fn analog_write(&mut self, pin: Pin, level: pin::Level) {
        self.0.analog_write(pin.value(), level as i32);
    }

    /// Set the given pin of the Arduino to the given mode.
    pub fn set_pin_mode(&mut self, pin: Pin, mode: pin::Mode) {
        self.0.set_pin_mode(pin.value(), mode.value());
    }
}
