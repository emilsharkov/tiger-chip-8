use std::path::{Path, PathBuf};

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(name = "tiger-chip8")]
#[command(about = "A CHIP-8 emulator written in Rust")]
#[command(version)]
pub struct Args {
    #[arg(value_name = "ROM_FILE", value_parser = validate_rom_file_parser)]
    pub rom_file: PathBuf,
    
    #[arg(short, long, default_value = "15", value_parser = validate_scale_parser)]
    pub scale: u8,
    
    #[arg(short, long, default_value = "10", value_parser = validate_ticks_per_frame_parser)]
    pub ticks_per_frame: usize,
}

fn validate_rom_file_parser(s: &str) -> Result<PathBuf, String> {
    let rom_file = PathBuf::from(s);
    if rom_file == Path::new(".") {
        return Err("Path cannot be '.'".to_string());
    }
    if !rom_file.exists() {
        return Err("File does not exist".to_string());
    }
    if !rom_file.is_file() {
        return Err("Path is not a file".to_string());
    }
    Ok(rom_file)
}

fn validate_scale_parser(s: &str) -> Result<u8, String> {
    let scale = s.parse::<u8>().map_err(|e| format!("Invalid scale format: {}", e))?;
    if !(scale >= 1 && scale <= 20) {
        return Err("Scale must be between 1 and 20".to_string());
    }
    Ok(scale)
}

fn validate_ticks_per_frame_parser(s: &str) -> Result<usize, String> {
    let ticks_per_frame = s.parse::<usize>().map_err(|e| format!("Invalid ticks per frame format: {}", e))?;
    if !(ticks_per_frame >= 1 && ticks_per_frame <= 100) {
        return Err("Ticks per frame must be between 1 and 100".to_string());
    }
    Ok(ticks_per_frame)
}
