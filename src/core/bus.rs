use crate::core::{display::Display, memory::Memory, input::Input, timers::Timers, speaker::Speaker};

pub struct Bus<'a> {
    pub memory: &'a mut Memory,
    pub display: &'a mut Display,
    pub input: &'a Input,
    pub timers: &'a mut Timers,
    pub speaker: &'a mut Speaker,
}