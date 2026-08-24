use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter};

pub struct AudioCapture {
    is_running: Arc<AtomicBool>,
    _stream: Option<cpal::Stream>,
}

impl AudioCapture {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            _stream: None,
        }
    }

    pub fn start(&mut self, app: AppHandle) -> Result<(), String> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("Nenhum microfone encontrado")?;

        let config = device
            .default_input_config()
            .map_err(|e| format!("Erro ao pegar config do microfone: {}", e))?;

        let sample_rate = config.sample_rate();
        let channels = config.channels() as usize;

        let is_running = self.is_running.clone();
        is_running.store(true, Ordering::SeqCst);

        let err_fn = |err| eprintln!("Erro no stream de áudio: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let volume = calculate_rms(data);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let data_f32: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    let volume = calculate_rms(&data_f32);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_input_stream(
                config.into(),
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    if !is_running.load(Ordering::SeqCst) {
                        return;
                    }
                    let data_f32: Vec<f32> = data
                        .iter()
                        .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                        .collect();
                    let volume = calculate_rms(&data_f32);
                    let _ = app.emit("audio-volume", volume);
                },
                err_fn,
                None,
            ),
            _ => return Err("Formato de sample não suportado".into()),
        }
        .map_err(|e| format!("Erro ao criar stream: {}", e))?;

        stream.play().map_err(|e| format!("Erro ao iniciar stream: {}", e))?;

        self._stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::SeqCst);
        self._stream = None;
    }
}

fn calculate_rms(data: &[f32]) -> f32 {
    if data.is_empty() {
        return 0.0;
    }
    let sum: f32 = data.iter().map(|&s| s * s).sum();
    let rms = (sum / data.len() as f32).sqrt();

    // Normaliza e aplica um ganho (ajuste esse valor se ficar muito sensível ou pouco)
    let volume = (rms * 8.0).clamp(0.0, 1.0);

    // Threshold de silêncio (abaixo disso considera boca fechada)
    if volume < 0.03 {
        0.0
    } else {
        volume
    }
}