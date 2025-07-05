pub mod gui;
pub mod emulator;
pub mod apu;
pub mod display;
use tiger_chip_8_core::vram::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_width() -> usize {
    DISPLAY_WIDTH
}

#[wasm_bindgen]
pub fn get_height() -> usize {
    DISPLAY_HEIGHT
}