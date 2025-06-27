use std::fs::File;
use sdl2::keyboard::Keycode;
use std::io::Read;
use crate::core::bus::Bus;
use crate::desktop::display::DesktopDisplay;
use crate::core::emulator::Emulator;
use crate::core::cpu::Cpu;
use crate::core::ram::Ram;
use crate::core::vram::Vram;
use crate::core::keypad::Keypad;
use crate::core::timers::Timers;

pub struct DesktopEmulator {
    cpu: Cpu,
    ram: Ram,
    vram: Vram,
    keypad: Keypad,
    timers: Timers,
    display: DesktopDisplay,
}

impl Emulator<DesktopDisplay, Keycode> for DesktopEmulator {
    fn new(
        cpu: Cpu,
        ram: Ram,
        vram: Vram,
        keypad: Keypad,
        timers: Timers,
        display: DesktopDisplay,
    ) -> Self {
        Self { cpu, ram, vram, keypad, timers, display }
    }

    fn tick_frame(&mut self) {
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

    fn tick_cycle(&mut self) {
        // TODO: add sound
        self.timers.tick();
    }

    fn load_rom(&mut self, file_path: &str) -> Vec<u8> {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        buffer
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
}
