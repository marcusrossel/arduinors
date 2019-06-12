use crate::cli;
pub use cli::Error;

pub use crate::pin;
pub use crate::pin::Pin;

/// A handle on an Arduino, to communicate with it via the Firmata protocol.
pub struct Arduino {
    board: firmata::Board
}

impl Arduino {

    /// Creates an Arduino object from the single currently connected Arduino device.
    ///
    /// # Errors
    /// This function calls `arduino::cli::query`, and will pass along any errors produced by it.
    pub fn new() -> Result<Arduino, Error> {
        let port = cli::query(cli::Query::Port)?;
        let arduino = Arduino { board: firmata::Board::new(&port[..]) };

        Ok(arduino)
    }

    pub fn digital_write(&mut self, pin: &Pin, state: &pin::State) {
        self.board.digital_write(pin.value(), state.value());
    }

    pub fn analog_write(&mut self, pin: &Pin, level: pin::Level) {
        self.board.analog_write(pin.value(), level as i32);
    }

    pub fn set_pin_mode(&mut self, pin: &Pin, mode: &pin::Mode) {
        self.board.set_pin_mode(pin.value(), mode.value());
    }
}
