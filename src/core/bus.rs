use crate::core::{vram::Vram, ram::Ram, keypad::Keypad, timers::Timers};

pub struct Bus<'a> {
    pub ram: &'a mut Ram,
    pub vram: &'a mut Vram,
    pub keypad: &'a Keypad,
    pub timers: &'a mut Timers,
}