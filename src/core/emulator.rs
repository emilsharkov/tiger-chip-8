use crate::core::cpu::Cpu;
use crate::core::ram::Ram;
use crate::core::vram::Vram;
use crate::core::keypad::Keypad;
use crate::core::timers::Timers;

pub trait Emulator<Display, Control> {
    fn new(
        cpu: Cpu,
        ram: Ram,
        vram: Vram,
        keypad: Keypad,
        timers: Timers,
        display: Display,
    ) -> Self;

    fn tick_frame(&mut self);
    fn tick_cycle(&mut self);
    fn run(&mut self);
    fn load_rom(&mut self, file_path: &str) -> Vec<u8>;
    fn to_keycode(&mut self, control: Control) -> Option<u8>;
}
