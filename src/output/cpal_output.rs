use crate::output::output::Output;
use crate::pipeline::{Pipeline, PlayerState};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use std::sync::Arc;


/// Implémentation de la sortie audio utilisant la bibliothèque CPAL.
/// 
/// Cette structure gère la connexion avec le périphérique de sortie par défaut
/// et contrôle le flux audio en fonction de l'état du pipeline.
pub struct CpalOutput {
    /// L'hôte audio (ASIO, CoreAudio, ALSA, etc.).
    host: cpal::Host,
    /// Le périphérique de sortie audio physique.
    device: cpal::Device,
    /// Configuration du flux (fréquence d'échantillonnage, canaux, etc.).
    pub config: cpal::StreamConfig,
    /// Le flux audio actif, s'il existe.
    pub stream: Option<Stream>,
    /// Référence partagée vers le pipeline de contrôle.
    pub pipeline: Arc<Pipeline>,
}


impl CpalOutput {
    /// Crée une nouvelle instance de `CpalOutput` liée à un `Pipeline`.
    /// 
    /// Tente d'ouvrir le périphérique de sortie par défaut du système.
    pub fn new(pipeline : Arc<Pipeline>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device");
        let config = device.default_output_config().unwrap().into();
        CpalOutput { host, device, config, stream: None, pipeline }
    }
}


impl Output for CpalOutput {
    /// Démarre la lecture des échantillons audio.
    /// 
    /// Cette méthode crée un thread de rappel (callback) audio qui consulte 
    /// le pipeline à chaque cycle pour ajuster le volume ou mettre en pause.
    fn play(&mut self, samples: Vec<f32>) -> Result<(), String> {
        let samples = Arc::new(samples);
        let mut index = 0;
        let pipeline = Arc::clone(&self.pipeline);

        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let volume = pipeline.get_volume();
                let state = pipeline.get_state();

                for output_sample in data.iter_mut() {
                    match state {
                        PlayerState::Playing => {
                            if index < samples.len() {
                                *output_sample = samples[index] * volume;
                                index += 1;
                            } else {
                                *output_sample = 0.0;
                            }
                        }
                        PlayerState::Paused => {
                            // En pause : on envoie du silence sans avancer dans les données.
                            *output_sample = 0.0; 
                        }
                        PlayerState::Stopped => {
                            // Arrêté : silence et remise à zéro de l'index.
                            *output_sample = 0.0;
                            index = 0;           
                        }
                    }
                }
            },
            move |err| {eprintln!("Stream error: {}", err);},
            None,
        ).map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Demande la mise en pause via le pipeline.
    fn pause(&mut self) -> Result<(), String> {
        self.pipeline.pause();
        Ok(())
    }

    /// Demande la reprise de la lecture via le pipeline.
    fn replay(&mut self) -> Result<(), String> {
        self.pipeline.play();
        Ok(())
    }

    /// Arrête la lecture et réinitialise la position via le pipeline.
    fn stop(&mut self) -> Result<(), String> {
        self.pipeline.stop();
        Ok(())
    }

    /// Modifie le volume global via le pipeline.
    fn set_volume(&mut self, volume: f32) {
        self.pipeline.set_volume(volume);
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::Pipeline;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_cpal_output_initialization() {
        // Création du pipeline nécessaire à CpalOutput
        let pipeline = Arc::new(Pipeline::new());
        let output = CpalOutput::new(Arc::clone(&pipeline));

        // Vérifier que le périphérique et la config sont bien là
        assert!(output.stream.is_none()); // Pas de flux au démarrage
        assert_eq!(output.pipeline.get_volume(), 1.0); // Volume par défaut du Pipeline
    }

    #[test]
    fn test_play_creates_stream() {
        let pipeline = Arc::new(Pipeline::new());
        let mut output = CpalOutput::new(Arc::clone(&pipeline));

        // Simuler quelques échantillons de silence
        let samples = vec![0.0; 441]; 

        // Lancer la lecture
        let result = output.play(samples);
        
        // Vérifier que le flux a bien été créé
        assert!(result.is_ok());
        assert!(output.stream.is_some());
    }

    #[test]
    fn test_output_commands_update_pipeline() {
        let pipeline = Arc::new(Pipeline::new());
        let mut output = CpalOutput::new(Arc::clone(&pipeline));

        // Tester si l'appel sur l'output modifie bien le pipeline
        output.pause().unwrap();
        thread::sleep(Duration::from_millis(50));
        assert_eq!(pipeline.get_state(), crate::pipeline::PlayerState::Paused);

        output.set_volume(0.2);
        thread::sleep(Duration::from_millis(50));
        assert_eq!(pipeline.get_volume(), 0.2);
    }
}