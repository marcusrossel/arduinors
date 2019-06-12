//! This module contains types used for type-safe modelling of an Arduino's digital pins and their
//! associated properties like state, mode and level.

/// A digital pin on an Arduino.
pub struct Pin {
    value: i32
}

impl Pin {
    /// Creates a pin from its number value.
    /// Valid values are 2, 3, ... 11, 12.
    pub fn new(value: i32) -> Pin {
        if value < 2 || value > 12 {
            panic!("Pin initializer received invalid value: {}", value);
        }

        Pin { value }
    }

    pub fn value(&self) -> i32 { self.value }
}

/// The mode of a digital pin on an Arduino.
pub enum Mode { Input, Output }

impl Mode {
    /// Returns a numeric equivalent of a pin mode.
    pub fn value(&self) -> u8 {
        match self { Mode::Input => 0, Mode::Output => 1 }
    }
}

/// The state of a digital pin on an Arduino.
pub enum State { Low, High }

impl State {
    /// Returns a numeric equivalent of a pin state.
    pub fn value(&self) -> i32 {
        match self { State::Low => 0, State::High => 1 }
    }
}

/// A PWM level of a digital pin on an Arduino.
pub type Level = u8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pin() {
        let pin_10 = Pin::new(10);
        assert_eq!(pin_10.value(), 10);
    }

    #[test]
    #[should_panic]
    fn invalid_pin() {
        let _pin_1 = Pin::new(1);
    }

    #[test]
    fn mode_value() {
        assert_eq!(Mode::Input.value(), 0);
        assert_eq!(Mode::Output.value(), 1);
    }

    #[test]
    fn state_value() {
        assert_eq!(State::Low.value(), 0);
        assert_eq!(State::High.value(), 1);
    }
}
