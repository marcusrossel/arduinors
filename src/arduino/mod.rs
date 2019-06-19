mod board;
pub use board::*;

use std::ops::Range;

/// A digital pin on an Arduino.
#[derive(Clone, Debug)]
pub struct DigitalPin {
    mode: PinMode,
    bit_resolution: u8,
    valid_modes: Vec<PinMode>,
}

impl DigitalPin {

    /// The mode of the pin on the source Arduino (at the time that this instance was retrieved).
    pub fn mode(&self) -> PinMode { self.mode }

    /// The range of values which are valid for the pin in its current mode.
    pub fn valid_values(&self) -> Range<i32> {
        0..(2i32.pow(self.bit_resolution as u32))
    }

    /// Constructs a digital pin instance from a non-analog (digital) `firmata::Pin`.
    ///
    /// # Panics
    /// * should never panic, but could if there is an implementation error.
    fn from_digital(firmata_pin: &firmata::Pin) -> DigitalPin {
        let mode = PinMode::from(firmata_pin.mode);
        let mut valid_modes: Vec<PinMode> = vec![];
        let mut bit_resolution: Option<u8> = None;

        for firmata_mode in firmata_pin.modes.iter() {
            if firmata_mode.mode == firmata_pin.mode {
                if bit_resolution.is_none() {
                    bit_resolution = Some(firmata_mode.resolution);
                } else {
                    panic!("Internal inconsistency between arduino::DigitalPin and firmata::Pin");
                }
            }

            valid_modes.push(PinMode::from(firmata_mode.mode));
        }

        if valid_modes.is_empty() { bit_resolution = Some(0); }

        if let Some(bit_resolution) = bit_resolution {
            DigitalPin { mode, bit_resolution, valid_modes }
        } else {
            panic!("Internal inconsistency between arduino::DigitalPin and firmata::Pin");
        }
    }
}

/// The mode of a pin on an Arduino.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PinMode {
    DigitalInput  = 0x0,
    DigitalOutput = 0x1,
    AnalogInput   = 0x2,
    Pwm           = 0x3,
    Servo         = 0x4,
    Shift         = 0x5,
    I2c           = 0x6,
    OneWire       = 0x7,
    Stepper       = 0x8,
    Encoder       = 0x9,
    Serial        = 0xA,
    InputPullup   = 0xB,
}

impl PinMode {

    /// Constructs a pin mode from its raw value.
    ///
    /// # Panics
    /// * if the given value does not correspond to one of the raw values of the enum's variants.
    fn from(value: u8) -> PinMode {
        match value {
            0x0 => PinMode::DigitalInput ,
            0x1 => PinMode::DigitalOutput,
            0x2 => PinMode::AnalogInput,
            0x3 => PinMode::Pwm,
            0x4 => PinMode::Servo,
            0x5 => PinMode::Shift,
            0x6 => PinMode::I2c,
            0x7 => PinMode::OneWire,
            0x8 => PinMode::Stepper,
            0x9 => PinMode::Encoder,
            0xA => PinMode::Serial,
            0xB => PinMode::InputPullup,
              _ => panic!(format!("PinMode can not be constructed from value '{}'", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pin_value() {
        let pin = DigitalPin { mode: PinMode::Pwm, bit_resolution: 10, valid_modes: vec![] };

        assert_eq!(pin.valid_values(), 0..1024);
    }

    #[test]
    fn invalid_pin_value() {
        let pin = DigitalPin { mode: PinMode::Serial, bit_resolution: 1, valid_modes: vec![] };

        assert!(!pin.valid_values().contains(&2));
    }
}
