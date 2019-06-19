use std::sync::mpsc;

use crate::Board;
use crate::arduino::DigitalPin;
use crate::arduino::PinMode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    InvalidPinIndex,
    ValueOutOfBounds,
    InvalidMode,
    Unimplemented,
}

/// A handle on an Arduino, for communicating with it via the Firmata protocol.
pub struct Arduino {
    board: firmata::Board,
    digital_pins: Vec<DigitalPin>,
}

impl Arduino {

    /// Creates an Arduino bound to a given board.
    pub fn from(board: &Board) -> Arduino {
        let board = firmata::Board::new(board.port());
        let digital_pins = Arduino::digital_pins_for_board(&board);

        Arduino { board, digital_pins }
    }

    /// Converts the `firmata::Board`'s collection of `firmata::Pin`s to a collection of
    /// `arduino::Pin`s.
    fn digital_pins_for_board(board: &firmata::Board) -> Vec<DigitalPin> {
        let (initial_tx, mut rx) = mpsc::channel::<Vec<DigitalPin>>();

        initial_tx.send(vec![])
            .expect("Sending to MPSC channel failed.");

        for firmata_pin in board.pins.iter().filter(|pin| !pin.analog ) {
            let current_rx = rx;
            let (current_tx, next_rx) = mpsc::channel();
            rx = next_rx;

            crossbeam::thread::scope(|scope| {
                scope.spawn(move |_| {
                    let digital_pin = DigitalPin::from_digital(firmata_pin);
                    let mut pin_list = current_rx.recv()
                        .expect("MPSC channel chain failed.");

                    pin_list.push(digital_pin);
                    current_tx.send(pin_list)
                        .expect("Sending to MPSC channel failed.");
                });
            })
            .expect("Crossbeam scope failed.");
        }

        rx.recv()
            .expect("MPSC channel chain failed.")
    }

    /// A collection of the digital pins for this Arduino.
    pub fn digital_pins(&self) -> &Vec<DigitalPin> { &self.digital_pins }

    pub fn write(&mut self, pin_index: i32, value: i32) -> Result<(), Error> {
        if let Some(pin) = self.digital_pins.get(pin_index as usize) {
            if pin.valid_values().contains(&value) {
                match pin.mode() {
                    PinMode::DigitalOutput => self.board.digital_write(pin_index, value),
                    PinMode::Pwm => self.board.analog_write(pin_index, value),
                    _ => return Err(Error::Unimplemented),
                }

                self.digital_pins= Arduino::digital_pins_for_board(&self.board);
                Ok(())
            } else {
                Err(Error::ValueOutOfBounds)
            }
        } else {
            Err(Error::InvalidPinIndex)
        }
    }

    pub fn set_pin_mode(&mut self, pin_index: i32, mode: PinMode) -> Result<(), Error> {
        if let Some(pin) = self.digital_pins.get(pin_index as usize) {
            if pin.valid_modes.contains(&mode) {
                self.board.set_pin_mode(pin_index, mode as u8);

                self.digital_pins= Arduino::digital_pins_for_board(&self.board);
                Ok(())
            } else {
                Err(Error::InvalidMode)
            }
        } else {
            Err(Error::InvalidPinIndex)
        }
    }
}
