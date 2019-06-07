// A digital pin on an Arduino.
pub enum Pin { P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12 }

impl Pin {
    pub fn value(&self) -> i32 {
        match self {
            Pin::P2 => 2, Pin::P3 => 3, Pin::P4 => 4, Pin::P5 => 5, Pin::P6 => 6, Pin::P7 => 7,
            Pin::P8 => 8, Pin::P9 => 9, Pin::P10 => 10, Pin::P11 => 11, Pin::P12 => 12
        }
    }
}

// The mode of a digital pin on an Arduino.
pub enum Mode { Input, Output }

impl Mode {
    pub fn value(&self) -> u8 {
        match self { Mode::Input => 0, Mode::Output => 1 }
    }
}

// The state of a digital pin on an Arduino.
pub enum State { Low, High }

impl State {
    pub fn value(&self) -> i32 {
        match self { State::Low => 0, State::High => 1 }
    }
}

// A PWM level of a digital pin on an Arduino.
pub type Level = u8;
