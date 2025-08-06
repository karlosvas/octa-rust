use cpal::{
    Device, Host, Stream, StreamError, SupportedStreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::sync::{Arc, mpsc};

use crate::widgets::notes::Note;

pub fn get_frecuency(note: &mut Note) {
    // Inicializa el host y el dispositivo de entrada
    let host: Host = cpal::default_host();
    let device: Device = host
        .default_input_device()
        .expect("No input device available");
    let config: SupportedStreamConfig = device.default_input_config().unwrap();
    let sample_rate: f32 = config.sample_rate().0 as f32;

    // Canal para enviar muestras
    let (tx, rx) = mpsc::channel();

    // Stream de entrada
    let stream: Stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    tx.send(sample).ok();
                }
            },
            move |err: StreamError| {
                eprintln!("Error: {:?}", err);
            },
            None, // Add the missing fourth argument for stream configuration
        )
        .unwrap();

    stream.play().unwrap();

    // Buffer para FFT
    let mut samples: Vec<Complex<f32>> = Vec::new();
    let fft_size: usize = 1024;
    let mut planner: FftPlanner<f32> = FftPlanner::new();
    let fft: Arc<dyn Fft<f32>> = planner.plan_fft_forward(fft_size);

    // Recoge muestras
    while samples.len() < fft_size {
        if let Ok(sample) = rx.recv() {
            samples.push(Complex::new(sample, 0.0));
        }
    }

    // Ejecuta FFT
    fft.process(&mut samples);

    // Calcular energía total del buffer
    let energy: f32 = samples.iter().map(|c: &Complex<f32>| c.norm()).sum();
    let energy_threshold: f32 = 50.0; // Puedes ajustar este valor si lo necesitas

    if energy > energy_threshold {
        // Encuentra la frecuencia dominante
        let max_index: usize = samples
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.norm().partial_cmp(&b.1.norm()).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let freq = max_index as f32 * sample_rate / fft_size as f32;

        // Filtrar solo frecuencias dentro del rango de un piano real (A0 a C8)
        let midi_note: f32 = (69.0 + 12.0 * (freq / 440.0).log2()).round();
        if freq >= 27.5 && freq <= 4186.0 && note.pitch == midi_note as u8 {
            note.is_active = true;
        }
    }
}
