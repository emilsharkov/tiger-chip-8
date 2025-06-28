use sdl2::audio::{AudioCallback, AudioDevice};
use tiger_chip8_core::apu::Apu;

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

pub struct SquareWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}