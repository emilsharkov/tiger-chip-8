use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{js_sys, Blob, CanvasRenderingContext2d, HtmlAudioElement, Url};

const AUDIO_FREQUENCY: u32 = 44100;
const SQUARE_WAVE_VOLUME: f32 = 0.25;
const SQUARE_WAVE_FREQUENCY: f32 = 440.0;
const SQUARE_WAVE_THRESHOLD: f32 = 0.5;
const PHASE_WRAP: f32 = 1.0;
const LOOP_DURATION_SECS: u32 = 1;

#[wasm_bindgen]
pub fn get_canvas_context(width: u32, height: u32, scale: u8) -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let scaled_width = width * scale as u32;
    let scaled_height = height * scale as u32;
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_attribute("width", &scaled_width.to_string()).unwrap();
    canvas.set_attribute("height", &scaled_height.to_string()).unwrap();

    let ctx = canvas.get_context("2d").unwrap().unwrap();
    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
    ctx
}

#[wasm_bindgen]
pub fn get_audio() -> HtmlAudioElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let audio = document.get_element_by_id("audio").unwrap();
    let audio = audio
        .dyn_into::<web_sys::HtmlAudioElement>()
        .map_err(|_| ())
        .unwrap();
    generate_and_bind_square_wave(&audio);
    audio
}

fn generate_and_bind_square_wave(audio: &HtmlAudioElement) {
    let sample_count = AUDIO_FREQUENCY * LOOP_DURATION_SECS;
    let mut samples = Vec::with_capacity(sample_count as usize);

    let mut phase = 0.0;
    let phase_increment = SQUARE_WAVE_FREQUENCY / AUDIO_FREQUENCY as f32;

    for _ in 0..sample_count {
        let sample = if phase < SQUARE_WAVE_THRESHOLD {
            (SQUARE_WAVE_VOLUME * i16::MAX as f32) as i16
        } else {
            (-SQUARE_WAVE_VOLUME * i16::MAX as f32) as i16
        };

        samples.push(sample);

        phase += phase_increment;
        if phase >= PHASE_WRAP {
            phase -= PHASE_WRAP;
        }
    }

    let wav_data = create_wav(samples, AUDIO_FREQUENCY);

    let array = js_sys::Uint8Array::from(&wav_data[..]);
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&array.buffer());
    let blob = Blob::new_with_u8_array_sequence(&blob_parts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    audio.set_src(&url);
}

fn create_wav(samples: Vec<i16>, sample_rate: u32) -> Vec<u8> {
    let num_channels = 1;
    let bits_per_sample = 16;
    let byte_rate = sample_rate * num_channels * bits_per_sample / 8;
    let block_align = num_channels * bits_per_sample / 8;
    let data_len = (samples.len() * 2) as u32;

    let mut wav = vec![];

    wav.extend(b"RIFF");
    wav.extend(&(36 + data_len).to_le_bytes());
    wav.extend(b"WAVE");

    wav.extend(b"fmt ");
    wav.extend(&(16u32).to_le_bytes()); // Subchunk1Size
    wav.extend(&(1u16).to_le_bytes());  // AudioFormat = PCM
    wav.extend(&(num_channels as u16).to_le_bytes());
    wav.extend(&(sample_rate).to_le_bytes());
    wav.extend(&(byte_rate).to_le_bytes());
    wav.extend(&(block_align as u16).to_le_bytes());
    wav.extend(&(bits_per_sample as u16).to_le_bytes());

    wav.extend(b"data");
    wav.extend(&(data_len).to_le_bytes());
    for s in samples {
        wav.extend(&s.to_le_bytes());
    }

    wav
}