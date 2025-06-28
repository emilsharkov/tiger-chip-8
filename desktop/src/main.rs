mod apu;
mod args;
mod display;
mod emulator;
mod gui;

use crate::{apu::DesktopApu, args::Args, gui::init_gui};
use clap::Parser;
use display::DesktopDisplay;
use emulator::DesktopEmulator;
use sdl2::event::Event;
use std::{fs::File, io::Read, path::PathBuf};
use tiger_chip8_core::{
    apu::Apu,
    cpu::Cpu,
    display::Display,
    emulator::Emulator,
    keypad::Keypad,
    ram::Ram,
    timers::Timers,
    vram::{Vram, DISPLAY_HEIGHT, DISPLAY_WIDTH},
};

fn main() {
    let Args {
        rom_file,
        scale,
        ops_per_frame,
    } = args::Args::try_parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let rom_bytes = get_rom_bytes(rom_file);
    let (mut event_pump, canvas, audio_device) =
        init_gui(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, scale.into());

    let mut emulator = DesktopEmulator::new(
        Cpu::new(),
        Ram::new(),
        Vram::new(),
        Keypad::new(),
        Timers::new(),
        DesktopDisplay::new(canvas),
        DesktopApu::new(audio_device),
    );
    emulator.load_rom(rom_bytes);
    emulator.load_font_set();

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'game_loop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = emulator.to_keycode(key) {
                        emulator.handle_key_press(k.into(), true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = emulator.to_keycode(key) {
                        emulator.handle_key_press(k.into(), false);
                    }
                }
                _ => {}
            }
        }
        for _ in 0..ops_per_frame {
            emulator.emulate_instruction();
        }
        emulator.tick_timers();
        emulator.draw_screen(DISPLAY_WIDTH, scale);
    }
}

fn get_rom_bytes(rom_file: PathBuf) -> Vec<u8> {
    let mut file = File::open(rom_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
