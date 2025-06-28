use crate::architecture::cpu::Cpu;
use crate::architecture::keypad::Keypad;
use crate::architecture::ram::Ram;
use crate::architecture::timers::Timers;
use crate::architecture::vram::Vram;

pub trait Emulator<Display, Apu, Control> {
    fn new(
        cpu: Cpu,
        ram: Ram,
        vram: Vram,
        keypad: Keypad,
        timers: Timers,
        display: Display,
        apu: Apu,
    ) -> Self;

    fn emulate_instruction(&mut self);
    fn tick_timers(&mut self);
    fn load_rom(&mut self, rom_bytes: Vec<u8>);
    fn load_font_set(&mut self);
    fn to_keycode(&mut self, control: Control) -> Option<u8>;
    fn handle_key_press(&mut self, key: usize, pressed: bool);
    fn draw_screen(&mut self, width: usize, scale: u8);
}
