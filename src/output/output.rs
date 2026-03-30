use cpal::traits::HostTrait;


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