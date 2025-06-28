mod emulator;
mod display;
mod args;
use clap::Parser;
use emulator::DesktopEmulator;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use tiger_chip8_core::emulator::Emulator;
use tiger_chip8_core::display::Display;
use tiger_chip8_core::{cpu::Cpu, ram::Ram, keypad::Keypad, timers::Timers, vram::Vram, vram::DISPLAY_WIDTH, vram::DISPLAY_HEIGHT};
use display::DesktopDisplay;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args = args::Args::try_parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    println!("Args: {:?}", args);
    
    let rom_bytes = get_rom_bytes(args.rom_file);
    let (mut event_pump, canvas) = init_sdl(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, args.scale.into());

    let mut emulator = DesktopEmulator::new(
        Cpu::new(), 
        Ram::new(), 
        Vram::new(), 
        Keypad::new(), 
        Timers::new(), 
        DesktopDisplay::new(canvas),
    );
    emulator.load_rom(rom_bytes);

    // 'game_loop: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } => {
    //                 break 'game_loop;
    //             }
    //             Event::KeyDown { keycode: Some(key), .. } => {
    //                 if let Some(k) = emulator.to_keycode(key) {
    //                     emulator.handle_key_press(k.into(), true);
    //                 }
    //             }
    //             Event::KeyUp { keycode: Some(key), .. } => {
    //                 if let Some(k) = emulator.to_keycode(key) {
    //                     emulator.handle_key_press(k.into(), false);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //     for _ in 0..ticks_per_frame {
    //         emulator.tick_frame();
    //     }
    //     emulator.tick_cycle();
    //     emulator.draw_screen(DISPLAY_WIDTH, scale);
    // }
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
    let window = video_subsystem.window("CHIP-8 Emulator", width * scale, height * scale)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    (event_pump, canvas)
}