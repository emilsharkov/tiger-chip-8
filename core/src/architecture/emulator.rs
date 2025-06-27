use crate::architecture::cpu::Cpu;
use crate::architecture::ram::Ram;
use crate::architecture::vram::Vram;
use crate::architecture::keypad::Keypad;
use crate::architecture::timers::Timers;

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
    fn load_rom(&mut self, rom_bytes: Vec<u8>);
    fn to_keycode(&mut self, control: Control) -> Option<u8>;
}
