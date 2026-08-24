use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter, State};

pub struct AudioState {
    is_running: Arc<AtomicBool>,
    stream: Mutex<Option<cpal::Stream>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            stream: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn start_audio_capture(
    app: AppHandle,
    state: State<'_, AudioState>,
) -> Result<(), String> {
    if state.is_running.load(Ordering::SeqCst) {
        return Ok(());
    }

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| "Nenhum microfone encontrado".to_string())?;

    let config = device
        .default_input_config()
        .map_err(|e| format!("Erro ao obter configuração do microfone: {e}"))?;

    let is_running = state.is_running.clone();
    is_running.store(true, Ordering::SeqCst);

    let err_fn = |err| {
        eprintln!("[audio] Erro no stream: {err}");
    };

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let app = app.clone();
            let is_running = is_running.clone();
            device.build_input_stream(
                config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let volume = calculate_volume(data);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            )
        }
        cpal::SampleFormat::I16 => {
            let app = app.clone();
            let is_running = is_running.clone();
            device.build_input_stream(
                config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let samples: Vec<f32> = data
                        .iter()
                        .map(|&s| s as f32 / i16::MAX as f32)
                        .collect();
                    let volume = calculate_volume(&samples);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            )
        }
        cpal::SampleFormat::U16 => {
            let app = app.clone();
            let is_running = is_running.clone();
            device.build_input_stream(
                config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let samples: Vec<f32> = data
                        .iter()
                        .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                        .collect();
                    let volume = calculate_volume(&samples);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            )
        }
        sample_format => {
            return Err(format!("Formato de sample não suportado: {sample_format:?}"));
        }
    }
    .map_err(|e| format!("Erro ao criar stream de áudio: {e}"))?;

    stream
        .play()
        .map_err(|e| format!("Erro ao iniciar stream de áudio: {e}"))?;

    *state.stream.lock().unwrap() = Some(stream);

    Ok(())
}

#[tauri::command]
pub fn stop_audio_capture(state: State<'_, AudioState>) {
    state.is_running.store(false, Ordering::SeqCst);
    *state.stream.lock().unwrap() = None;
}

/// Calcula o volume normalizado (0.0 ~ 1.0) usando RMS
fn calculate_volume(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }

    let sum_squares: f32 = data.iter().map(|&s| s * s).sum();
    let rms = (sum_squares / data.len() as f32).sqrt();

    // Ganho — aumente se a boca abrir pouco, diminua se abrir demais
    let mut volume = (rms * 9.0).clamp(0.0, 1.0);

    // Threshold de silêncio (evita a boca tremer com ruído de fundo)
    if volume < 0.04 {
        volume = 0.0;
    }

    volume
}