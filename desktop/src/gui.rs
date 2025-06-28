use sdl2::{audio::{AudioDevice, AudioSpecDesired}, render::Canvas, video::Window, EventPump};

use crate::apu::SquareWave;

pub fn init_gui(width: u32, height: u32, scale: u32) -> (EventPump, Canvas<Window>, AudioDevice<SquareWave>) {
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
    let window = video_subsystem.window("CHIP-8 Emulator", width * scale, height * scale)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    canvas
}

fn get_audio_device(sdl_context: &sdl2::Sdl) -> AudioDevice<SquareWave> {
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            volume: 0.25,
            phase: 0.0,
            phase_inc: 440.0 / spec.freq as f32,
        }
    }).unwrap();
    audio_device.pause();
    audio_device
}
    