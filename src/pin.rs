//! This module contains types used for type-safe modelling of an Arduino's digital pins and their
//! associated properties like state, mode and level.

/// A digital pin on an Arduino.
#[derive(Clone, Copy)]
pub struct Pin(i32);

impl Pin {
    /// Creates a pin from its number value.
    /// Valid values are 2, 3, ... 11, 12.
    pub fn new(value: i32) -> Pin {
        if value < 2 || value > 12 {
            panic!("Pin initializer received invalid value: {}", value);
        }

        Pin(value)
    }

    pub fn value(&self) -> i32 { self.0 }
}

/// The mode of a digital pin on an Arduino.
#[derive(Clone, Copy)]
pub enum Mode { Input = 0, Output = 1 }

/// The state of a digital pin on an Arduino.
#[derive(Clone, Copy)]
pub enum State { Low = 0, High = 1 }

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
}
