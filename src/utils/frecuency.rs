use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::mpsc;

#[warn(dead_code)]
pub fn get_frecuency() {
    // Inicializa el host y el dispositivo de entrada
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("No input device available");
    let config = device.default_input_config().unwrap();
    let sample_rate = config.sample_rate().0 as f32;

    // Canal para enviar muestras
    let (tx, rx) = mpsc::channel();

    // Stream de entrada
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    tx.send(sample).ok();
                }
            },
            move |err| {
                eprintln!("Error: {:?}", err);
            },
            None, // Add the missing fourth argument for stream configuration
        )
        .unwrap();

    stream.play().unwrap();

    // Buffer para FFT
    let mut samples = Vec::new();
    let fft_size = 1024;
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);

    // Array de nombres de notas
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let note_names_es = [
        "Do", "Do#", "Re", "Re#", "Mi", "Fa", "Fa#", "Sol", "Sol#", "La", "La#", "Si",
    ];

    let mut last_midi_note: Option<i32> = None;

    // Recoge muestras
    while samples.len() < fft_size {
        if let Ok(sample) = rx.recv() {
            samples.push(Complex::new(sample, 0.0));
        }
    }

    // Ejecuta FFT
    fft.process(&mut samples);

    // Calcular energía total del buffer
    let energy: f32 = samples.iter().map(|c| c.norm()).sum();
    let energy_threshold = 50.0; // Puedes ajustar este valor si lo necesitas

    if energy > energy_threshold {
        loop {
            // Encuentra la frecuencia dominante
            let max_index = samples
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.norm().partial_cmp(&b.1.norm()).unwrap())
                .map(|(i, _)| i)
                .unwrap();
            let freq = max_index as f32 * sample_rate / fft_size as f32;

            // Filtrar solo frecuencias dentro del rango de un piano real (A0 a C8)
            if freq >= 27.5 && freq <= 4186.0 {
                // Convierte frecuencia a nota MIDI
                let midi_note = (69.0 + 12.0 * (freq / 440.0).log2()).round();
                let midi_note_int = midi_note as i32;

                // Convierte nota MIDI a nombre de nota de piano
                let note_index = ((midi_note_int % 12) + 12) % 12;
                let octave = (midi_note_int / 12) - 1;
                let note_name = note_names[note_index as usize];
                let note_name_es = note_names_es[note_index as usize];

                // Solo mostrar si la nota cambia
                if Some(midi_note_int) != last_midi_note {
                    println!(
                        "Frecuencia: {:.2} Hz, Nota MIDI: {} ({}{} / {}{})",
                        freq, midi_note, note_name, octave, note_name_es, octave
                    );
                    last_midi_note = Some(midi_note_int);
                }
            }
        }
    }
}
