use crate::cli;
pub use cli::Error;

pub use crate::pin;
pub use crate::pin::Pin;

// A handle on an Arduino, to communicate with it via the Firmata protocol.
pub struct Arduino { board: firmata::Board }

impl Arduino {

    // Creates an Arduino object from the single currently connected Arduino device, or Returns
    // an error if this is not possible.
    //
    // This function expects the Arduino CLI to be installed and accessible via the "arduino-cli"
    // command.
    pub fn new() -> Result<Arduino, Error> {
        let port = match cli::query(cli::Query::Port) {
            Ok(port) => port,
            Err(err) => return Err(err)
        };

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
