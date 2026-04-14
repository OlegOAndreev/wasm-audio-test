use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    /// Proxy for browser console.log()
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    /// Proxy for browser console.error()
    pub fn error(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        {
            crate::log(&format_args!($($t)*).to_string());
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!($($t)*);
        }
    }
}

#[allow(unused_macros)]
macro_rules! console_error {
    ($($t:tt)*) => {
        #[cfg(target_arch = "wasm32")]
        {
            crate::error(&format_args!($($t)*).to_string());
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            println!($($t)*);
        }
    }
}

const REQUESTED_SAMPLE_RATE: usize = 48000;
const NUM_CHANNELS: usize = 2;

const AMPLITUDE: f32 = 0.2;
const FREQ_1: f32 = 392.0;
const FREQ_2: f32 = 490.0;
const FREQ_3: f32 = 588.0;

struct TinyaudioStateInner {
    device: Option<tinyaudio::OutputDevice>,

    phase1: f32,
    phase2: f32,
    phase3: f32,
}

#[wasm_bindgen]
pub struct TinyaudioState(Arc<Mutex<TinyaudioStateInner>>);

impl Drop for TinyaudioState {
    fn drop(&mut self) {
        let mut inner = self.0.lock().unwrap();
        if let Some(mut device) = inner.device.take() {
            device.close();
        }
    }
}

#[wasm_bindgen]
pub fn init_tinyaudio() -> Result<TinyaudioState, JsError> {
    let params = tinyaudio::OutputDeviceParameters {
        sample_rate: REQUESTED_SAMPLE_RATE,
        channels_count: NUM_CHANNELS,
        channel_sample_count: 2048,
    };
    let state = Arc::new(Mutex::new(TinyaudioStateInner { device: None, phase1: 0.0, phase2: 0.0, phase3: 0.0 }));

    let state_clone = state.clone();
    let device = tinyaudio::run_output_device(params, move |output| {
        tinyaudio_callback(output, &mut state_clone.lock().unwrap());
    })
    .map_err(|e| JsError::new(&format!("Could not run_output_device: {}", e)))?;

    state.lock().unwrap().device = Some(device);

    Ok(TinyaudioState(state))
}

fn tinyaudio_callback(output: &mut [f32], state: &mut TinyaudioStateInner) {
    for frame in output.chunks_mut(NUM_CHANNELS) {
        let sample1 = state.phase1.sin() * AMPLITUDE;
        let sample2 = state.phase2.sin() * AMPLITUDE;
        let sample3 = state.phase3.sin() * AMPLITUDE;
        let mixed_sample = sample1 + sample2 + sample3;
        frame.fill(mixed_sample);

        state.phase1 += 2.0 * PI * FREQ_1 / REQUESTED_SAMPLE_RATE as f32;
        state.phase2 += 2.0 * PI * FREQ_2 / REQUESTED_SAMPLE_RATE as f32;
        state.phase3 += 2.0 * PI * FREQ_3 / REQUESTED_SAMPLE_RATE as f32;
        if state.phase1 >= 2.0 * PI {
            state.phase1 -= 2.0 * PI;
        }
        if state.phase2 >= 2.0 * PI {
            state.phase2 -= 2.0 * PI;
        }
        if state.phase3 >= 2.0 * PI {
            state.phase3 -= 2.0 * PI;
        }
    }
}

struct CPALStateInner {
    #[allow(unused)]
    device: cpal::Device,
    sample_rate: u32,
    stream: Option<cpal::Stream>,

    phase1: f32,
    phase2: f32,
    phase3: f32,
}

#[wasm_bindgen]
pub struct CPALState(Arc<Mutex<CPALStateInner>>);

impl Drop for CPALState {
    fn drop(&mut self) {
        // The streams in CPAL are stopped by dropping them.
        let mut state = self.0.lock().unwrap();
        state.stream = None;
    }
}

#[wasm_bindgen]
pub fn init_cpal() -> Result<CPALState, JsError> {
    let device = cpal::default_host().default_output_device();
    if device.is_none() {
        return Err(JsError::new(&format!("Could not default_output_device")));
    }
    let device = device.unwrap();
    let config = device
        .default_output_config()
        .map_err(|e| JsError::new(&format!("Could not default_output_config: {}", e)))?;
    console_log!("Got sample rate {} from CPAL", config.sample_rate());

    let state = Arc::new(Mutex::new(CPALStateInner {
        device: device.clone(),
        stream: None,
        sample_rate: config.sample_rate(),
        phase1: 0.0,
        phase2: 0.0,
        phase3: 0.0,
    }));

    let state_clone = state.clone();
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                cpal_callback(data, &mut state_clone.lock().unwrap());
            },
            |err| {
                console_error!("Error streaming: {}", err);
            },
            None,
        )
        .map_err(|e| JsError::new(&format!("Could not build_output_stream: {}", e)))?;
    stream.play().map_err(|e| JsError::new(&format!("Could not play: {}", e)))?;
    state.lock().unwrap().stream = Some(stream);
    Ok(CPALState(state))
}

fn cpal_callback(output: &mut [f32], state: &mut CPALStateInner) {
    for frame in output.chunks_mut(NUM_CHANNELS) {
        let sample1 = state.phase1.sin() * AMPLITUDE;
        let sample2 = state.phase2.sin() * AMPLITUDE;
        let sample3 = state.phase3.sin() * AMPLITUDE;
        let mixed_sample = sample1 + sample2 + sample3;
        frame.fill(mixed_sample);

        state.phase1 += 2.0 * PI * FREQ_1 / state.sample_rate as f32;
        state.phase2 += 2.0 * PI * FREQ_2 / state.sample_rate as f32;
        state.phase3 += 2.0 * PI * FREQ_3 / state.sample_rate as f32;
        if state.phase1 >= 2.0 * PI {
            state.phase1 -= 2.0 * PI;
        }
        if state.phase2 >= 2.0 * PI {
            state.phase2 -= 2.0 * PI;
        }
        if state.phase3 >= 2.0 * PI {
            state.phase3 -= 2.0 * PI;
        }
    }
}
