use sdl2::audio::AudioDevice;
use tiger_chip8_core::apu::Apu;

use crate::gui::SquareWave;

pub struct DesktopApu {
    audio_device: AudioDevice<SquareWave>,
}

impl Apu<AudioDevice<SquareWave>> for DesktopApu {
    fn new(audio_device: AudioDevice<SquareWave>) -> Self {
        Self { audio_device }
    }

    fn play(&self) {
        self.audio_device.resume()
    }

    fn stop(&self) {
        self.audio_device.pause()
    }
}
