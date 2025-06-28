pub trait Apu<AudioDevice> {
    fn new(audio_device: AudioDevice) -> Self;
    fn play(&self);
    fn stop(&self);
}
