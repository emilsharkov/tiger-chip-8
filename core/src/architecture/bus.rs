use crate::architecture::{keypad::Keypad, ram::Ram, timers::Timers, vram::Vram};

pub struct Bus<'a> {
    pub ram: &'a mut Ram,
    pub vram: &'a mut Vram,
    pub keypad: &'a Keypad,
    pub timers: &'a mut Timers,
}
