use crate::output::output::Output;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use std::sync::Arc;


pub struct CpalOutput {
    host: cpal::Host,
    device: cpal::Device,
    pub config: cpal::StreamConfig,
    pub stream: Option<Stream>,
    pub volume: f32,
}


impl CpalOutput {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device");
        let config = device.default_output_config().unwrap().into();
        let volume = 0.4;
        CpalOutput { host, device, config, stream: None, volume }
    }
}


impl Output for CpalOutput {
    fn play(&mut self, samples: Vec<f32>) -> Result<(), String> {
        let volume = self.volume;
        let samples = Arc::new(samples);

        let stream = self.device.build_output_stream(
            &self.config,
            {
                let samples = Arc::clone(&samples);
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    play_sample(data, &samples, volume);
                }
            },
            move |err| eprintln!("Stream error: {:?}", err),
            None,
        ).map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), String> {
        // Implémentation de la mise en pause
        Ok(())
    }

    fn replay(&mut self) -> Result<(), String> {
        // Implémentation de la reprise
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        // Implémentation de l'arrêt
        Ok(())
    }

    fn set_volume(&mut self, volume: f32) {
        // Implémentation du réglage du volume
    }


}

pub fn play_sample(data: &mut [f32], samples: &[f32], volume: f32) {
    for (output_sample, &sample) in data.iter_mut().zip(samples.iter()) {
        *output_sample = sample * volume;
    }
}