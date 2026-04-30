use cpal::traits::HostTrait;

/// Le trait `Output` définit l'interface standard pour tous les moteurs de sortie audio
/// de l'application.
///
/// Il permet d'abstraire la logique de manipulation du flux audio (lecture, pause, volume) ici CPAL
pub trait Output {
        
    /// jouer les échantillons audio
    fn play(&mut self, samples: Vec<f32>) -> Result<(), String>;

    /// mettre en pause la lecture
    fn pause(&mut self) -> Result<(), String>;

    /// reprendre la lecture après une pause
    fn replay(&mut self) -> Result<(), String>;

    /// arrêter la lecture
    fn stop(&mut self) -> Result<(), String>;

    /// régler le volume de la sortie audio
    fn set_volume(&mut self, volume: f32);


}