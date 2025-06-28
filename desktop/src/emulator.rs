use sdl2::keyboard::Keycode;
use tiger_chip8_core::apu::Apu;
use tiger_chip8_core::bus::Bus;
use tiger_chip8_core::emulator::Emulator;
use tiger_chip8_core::cpu::Cpu;
use tiger_chip8_core::ram::Ram;
use tiger_chip8_core::vram::Vram;
use tiger_chip8_core::keypad::Keypad;
use tiger_chip8_core::timers::Timers;
use tiger_chip8_core::architecture::display::Display;
use crate::display::DesktopDisplay;
use crate::apu::DesktopApu;

pub struct DesktopEmulator {
    cpu: Cpu,
    ram: Ram,
    vram: Vram,
    keypad: Keypad,
    timers: Timers,
    display: DesktopDisplay,
    apu: DesktopApu,
}

impl Emulator<DesktopDisplay, DesktopApu, Keycode> for DesktopEmulator {
    fn new(
        cpu: Cpu,
        ram: Ram,
        vram: Vram,
        keypad: Keypad,
        timers: Timers,
        display: DesktopDisplay,
        apu: DesktopApu,
    ) -> Self {
        Self { cpu, ram, vram, keypad, timers, display, apu }
    }

    fn emulate_instruction(&mut self) {
        let pc = self.cpu.program_counter as usize;
        let higher_byte = self.ram.read(pc) as u16;
        let lower_byte = self.ram.read(pc + 1) as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.cpu.program_counter += 2;
        self.cpu.execute_op(op, &mut Bus {
            ram: &mut self.ram,
            vram: &mut self.vram,
            keypad: &mut self.keypad,
            timers: &mut self.timers,
        });
    }

    fn tick_timers(&mut self) {
        self.timers.tick();
        if self.timers.sound_timer > 0 {
            self.apu.play();
        } else {
            self.apu.stop();
        }
    }

    fn load_rom(&mut self, rom_bytes: Vec<u8>) {
        self.ram.load_rom(rom_bytes);
    }

    fn load_font_set(&mut self) {
        self.ram.load_font_set();
    }
    
    fn to_keycode(&mut self, control: Keycode) -> Option<u8> {
        match control {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q    => Some(0x4),
            Keycode::W    => Some(0x5),
            Keycode::E    => Some(0x6),
            Keycode::R    => Some(0xD),
            Keycode::A    => Some(0x7),
            Keycode::S    => Some(0x8),
            Keycode::D    => Some(0x9),
            Keycode::F    => Some(0xE),
            Keycode::Z    => Some(0xA),
            Keycode::X    => Some(0x0),
            Keycode::C    => Some(0xB),
            Keycode::V    => Some(0xF),
            _             => None,
        }
    }

    fn handle_key_press(&mut self, key: usize, pressed: bool) {
        self.keypad.set_key(key as usize, pressed);
    }

    fn draw_screen(&mut self, width: usize, scale: u8) {
        self.display.draw_screen(&self.vram.pixels, width, scale);
    }
}
