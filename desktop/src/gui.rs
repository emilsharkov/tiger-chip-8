use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    render::Canvas,
    video::Window,
    EventPump,
};

// Audio constants
const AUDIO_FREQUENCY: i32 = 44100;
const AUDIO_CHANNELS: u8 = 1;
const SQUARE_WAVE_VOLUME: f32 = 0.25;
const SQUARE_WAVE_FREQUENCY: f32 = 440.0;
const SQUARE_WAVE_THRESHOLD: f32 = 0.5;
const PHASE_WRAP: f32 = 1.0;

pub fn init_gui(
    width: u32,
    height: u32,
    scale: u32,
) -> (EventPump, Canvas<Window>, AudioDevice<SquareWave>) {
    let sdl_context = sdl2::init().unwrap();
    let event_pump = get_event_pump(&sdl_context);
    let canvas = get_canvas(&sdl_context, width, height, scale);
    let audio_device = get_audio_device(&sdl_context);
    (event_pump, canvas, audio_device)
}

fn get_event_pump(sdl_context: &sdl2::Sdl) -> EventPump {
    sdl_context.event_pump().unwrap()
}

fn get_canvas(sdl_context: &sdl2::Sdl, width: u32, height: u32, scale: u32) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8 Emulator", width * scale, height * scale)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas
}

fn get_audio_device(sdl_context: &sdl2::Sdl) -> AudioDevice<SquareWave> {
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(AUDIO_FREQUENCY),
        channels: Some(AUDIO_CHANNELS),
        samples: None,
    };
    let audio_device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| SquareWave {
            volume: SQUARE_WAVE_VOLUME,
            phase: 0.0,
            phase_inc: SQUARE_WAVE_FREQUENCY / spec.freq as f32,
        })
        .unwrap();
    audio_device.pause();
    audio_device
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
            *x = if self.phase <= SQUARE_WAVE_THRESHOLD {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % PHASE_WRAP;
        }
    }
}
