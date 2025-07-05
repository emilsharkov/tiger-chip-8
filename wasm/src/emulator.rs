use crate::apu::WasmApu;
use crate::display::WasmDisplay;
use tiger_chip_8_core::{apu::Apu, emulator::Emulator};
use tiger_chip_8_core::architecture::display::Display;
use tiger_chip_8_core::bus::Bus;
use tiger_chip_8_core::cpu::Cpu;
use tiger_chip_8_core::keypad::Keypad;
use tiger_chip_8_core::ram::Ram;
use tiger_chip_8_core::timers::Timers;
use tiger_chip_8_core::vram::Vram;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlAudioElement};

#[wasm_bindgen]
pub struct WasmEmulator {
    cpu: Cpu,
    ram: Ram,
    vram: Vram,
    keypad: Keypad,
    timers: Timers,
    display: WasmDisplay,
    apu: WasmApu,
}

// This is the trait impl (not exported to JS)
impl Emulator<WasmDisplay, WasmApu, String> for WasmEmulator {
    fn new(
        cpu: Cpu,
        ram: Ram,
        vram: Vram,
        keypad: Keypad,
        timers: Timers,
        display: WasmDisplay,
        apu: WasmApu,
    ) -> Self {
        Self {
            cpu,
            ram,
            vram,
            keypad,
            timers,
            display,
            apu,
        }
    }

    fn emulate_instruction(&mut self) {
        let pc = self.cpu.program_counter as usize;
        let higher_byte = self.ram.read(pc) as u16;
        let lower_byte = self.ram.read(pc + 1) as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.cpu.program_counter += 2;
        self.cpu.execute_op(
            op,
            &mut Bus {
                ram: &mut self.ram,
                vram: &mut self.vram,
                keypad: &mut self.keypad,
                timers: &mut self.timers,
            },
        );
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

    fn to_keycode(&mut self, control: String) -> Option<u8> {
        match control.as_str() {
            "1" => Some(0x1),
            "2" => Some(0x2),
            "3" => Some(0x3),
            "4" => Some(0xC),
            "q" => Some(0x4),
            "w" => Some(0x5),
            "e" => Some(0x6),
            "r" => Some(0xD),
            "a" => Some(0x7),
            "s" => Some(0x8),
            "d" => Some(0x9),
            "f" => Some(0xE),
            "z" => Some(0xA),
            "x" => Some(0x0),
            "c" => Some(0xB),
            "v" => Some(0xF),
            _ => None,
        }
    }

    fn handle_key_press(&mut self, key: usize, pressed: bool) {
        self.keypad.set_key(key, pressed);
    }

    fn draw_screen(&mut self, width: usize, scale: u8) {
        self.display.draw_screen(&self.vram.pixels, width, scale);
    }
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        ctx: CanvasRenderingContext2d,
        audio: HtmlAudioElement,
    ) -> Self {
        let cpu = Cpu::new();
        let ram = Ram::new();
        let vram = Vram::new();
        let keypad = Keypad::new();
        let timers = Timers::new();
        let display = WasmDisplay::new(ctx);
        let apu = WasmApu::new(audio);

        Self {
            cpu,
            ram,
            vram,
            keypad,
            timers,
            display,
            apu,
        }
    }

    #[wasm_bindgen]
    pub fn emulate_instruction(&mut self) {
        Emulator::emulate_instruction(self);
    }

    #[wasm_bindgen]
    pub fn tick_timers(&mut self) {
        Emulator::tick_timers(self);
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom_bytes: Vec<u8>) {
        Emulator::load_rom(self, rom_bytes);
    }

    #[wasm_bindgen]
    pub fn load_font_set(&mut self) {
        Emulator::load_font_set(self);
    }

    #[wasm_bindgen]
    pub fn to_keycode(&mut self, control: String) -> Option<u8> {
        Emulator::to_keycode(self, control)
    }

    #[wasm_bindgen]
    pub fn handle_key_press(&mut self, key: usize, pressed: bool) {
        Emulator::handle_key_press(self, key, pressed);
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self, width: usize, scale: u8) {
        Emulator::draw_screen(self, width, scale);
    }
}
