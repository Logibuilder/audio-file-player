use audio_file_player::output::cpal_output::CpalOutput;
use audio_file_player::output::output::Output;

use std::thread;
use std::time::Duration;

#[test]
fn test_play_sine_wave() {
    let mut player = CpalOutput::new();

    /// crer une onde sinusoïdale de 440 Hz pendant 1 seconde
    let sample_rate = player.config.sample_rate as f32;
    let freq = 440.0; 
    let duration_sec = 1.0;
    let num_samples = (sample_rate * duration_sec) as usize;

    let samples: Vec<f32> = (0..num_samples)
        .map(|i| (2.0 * std::f32::consts::PI * freq * (i as f32 / sample_rate)).sin() * 0.5)
        .collect();

    player.play(samples).unwrap();

    // laisser le son jouer
    thread::sleep(Duration::from_secs_f32(duration_sec + 0.2));

    /// vérifier que le flux est actif
    assert!(player.stream.is_some());
}