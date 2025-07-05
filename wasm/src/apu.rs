use tiger_chip_8_core::apu::Apu;
use web_sys::HtmlAudioElement;

pub struct WasmApu {
    audio_device: HtmlAudioElement,
}

impl Apu<HtmlAudioElement> for WasmApu {
    fn new(audio_device: HtmlAudioElement) -> Self {
        Self { audio_device }
    }

    fn play(&self) {
        self.audio_device.set_loop(true);
        let _ = self.audio_device.play();
    }

    fn stop(&self) {
        let _ = self.audio_device.pause();
        let _ = self.audio_device.set_current_time(0.0);
    }
}