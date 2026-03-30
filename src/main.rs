use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    // 1. Lire un fichier WAV
    let mut reader = hound::WavReader::open("src/test.wav").unwrap();
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    let mut sample_index = 0;

    // 2. Setup CPAL
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let err_fn = |err| eprintln!("Erreur : {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            let config: cpal::StreamConfig = config.into();
            device.build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    for sample in data.iter_mut() {
                        if sample_index < samples.len() {
                            *sample = samples[sample_index];
                            sample_index += 1;
                        } else {
                            *sample = 0.0;
                        }
                    }
                },
                err_fn,
                None,
            )
        }
        _ => panic!("Format non supporté"),
    }
    .unwrap();

    stream.play().unwrap();

    println!("Lecture en cours...");

    std::thread::sleep(std::time::Duration::from_secs(5));
}