mod emulator;
mod display;

use clap::Parser;
use emulator::DesktopEmulator;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use tiger_chip8_core::emulator::Emulator;
use tiger_chip8_core::display::Display;
use tiger_chip8_core::{cpu::Cpu, ram::Ram, vram::Vram, keypad::Keypad, timers::Timers};
use display::DesktopDisplay;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Parser)]
#[command(name = "tiger-chip8")]
#[command(about = "A CHIP-8 emulator written in Rust")]
#[command(version)]
struct Args {
    /// Path to the CHIP-8 ROM file
    #[arg(value_name = "ROM_FILE")]
    rom_file: PathBuf,
    
    /// Display scale factor (default: 15)
    #[arg(short, long, default_value = "15")]
    scale: u32,
    
    /// CPU ticks per frame (default: 10)
    #[arg(short, long, default_value = "10")]
    ticks_per_frame: usize,
}

fn main() {
    let Args { rom_file, scale, ticks_per_frame } = parse_args();
    let rom_bytes = get_rom_bytes(rom_file);
    let (mut event_pump, canvas) = init_sdl(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, scale);

    let mut emulator = DesktopEmulator::new(
        Cpu::new(), 
        Ram::new(), 
        Vram::new(), 
        Keypad::new(), 
        Timers::new(), 
        DesktopDisplay::new(canvas),
    );
    emulator.load_rom(rom_bytes);

    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'game_loop;
                }

                _ => {}
            }
        }
    }
}

fn parse_args() -> Args {
    let args = Args::parse();
    if !args.rom_file.exists() {
        eprintln!("Error: ROM file '{}' does not exist", args.rom_file.display());
        std::process::exit(1);
    }
    args
}

fn get_rom_bytes(rom_file: PathBuf) -> Vec<u8> {
    let mut file = File::open(rom_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}


fn init_sdl(width: u32, height: u32, scale: u32) -> (EventPump, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("CHIP-8 Emulator", width, height)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    (event_pump, canvas)
}