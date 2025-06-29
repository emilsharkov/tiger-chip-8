use std::{fs::File, io::Read, path::PathBuf};

pub fn get_rom_bytes(rom_file: PathBuf) -> Vec<u8> {
    let mut file = File::open(rom_file).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}