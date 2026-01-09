use anyhow::{Context, Result};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SampleFormat};
use log::{error, info};
use std::sync::{Arc, Mutex};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

// VAD: RMS threshold. Optimize for edge: Low compute.
fn is_voice_active(samples: &[f32], threshold: f32) -> bool {
    let rms = samples.iter().map(|&s| s * s).sum::<f32>().sqrt() / samples.len() as f32;
    rms > threshold
}

/// Voice to Text using Whisper. MCP-inspired: Could pass audio context via protocol.
pub fn voice_to_text(model_path: &str) -> Result<String> {
    let host = cpal::default_host();
    let device = host.default_input_device().context("No input device")?;
    let config = device.default_input_config()?;

    if config.sample_format() != SampleFormat::F32 {
        return Err(anyhow::anyhow!("Only F32 supported"));
    }

    let samples: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let samples_clone = samples.clone();

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &_| {
            let mut guard = samples_clone.lock().unwrap();
            guard.extend_from_slice(data);
        },
        |err| error!("Audio error: {}", err),
        None,
    )?;

    stream.play()?;
    info!("Recording...");
    std::thread::sleep(std::time::Duration::from_secs(5));  // MVP fixed
    drop(stream);

    let audio = samples.lock().unwrap().clone();

    if !is_voice_active(&audio, 0.01) {
        return Ok("No voice detected".to_string());
    }

    let ctx = WhisperContext::new_with_params(model_path, Default::default()).context("Whisper load failed")?;
    let mut state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some("en"));

    state.full(params, &audio)?;
    let text = (0..state.full_n_segments()?)
        .map(|i| state.full_get_segment_text(i))
        .collect::<Result<Vec<_>, _>>()?
        .join(" ");

    info!("Transcribed: {}", text);
    Ok(text)
}