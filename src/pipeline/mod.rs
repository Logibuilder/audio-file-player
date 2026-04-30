use std::sync::{Arc, Mutex, mpsc::{Sender, channel}};
use std::thread;

/// Représente les commandes que l'on peut envoyer au lecteur.
#[derive(Debug, Clone)]
pub enum PlayerCommand {
    /// Lance ou reprend la lecture.
    Play,
    /// Suspend la lecture en gardant la position actuelle.
    Pause,
    /// Arrête la lecture et réinitialise la position.
    Stop,
    /// Modifie le volume sonore (0.0 à 1.0).
    SetVolume(f32),
}

/// Représente les différents états possibles du lecteur audio.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PlayerState {
    /// Le son est en cours de diffusion.
    Playing,
    /// La lecture est suspendue, le curseur temporel est maintenu.
    Paused,
    /// La lecture est arrêtée, le curseur est remis à zéro.
    Stopped,
}

/// Le Pipeline est le "cerveau" du lecteur. 
/// Il gère la communication entre l'interface utilisateur et le moteur audio.
pub struct Pipeline {
    /// Canal permettant d'envoyer des ordres au thread de contrôle.
    command_sender: Sender<PlayerCommand>,
    /// État actuel du lecteur, partagé en toute sécurité entre les threads.
    state: Arc<Mutex<PlayerState>>,
    /// Niveau du volume actuel (1.0 = 100%).
    volume: Arc<Mutex<f32>>,
}

impl Pipeline {
    /// Crée un nouveau Pipeline et lance son thread de contrôle interne.
    /// Par défaut, le lecteur commence en mode `Playing` avec un volume de 1.0.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let state = Arc::new(Mutex::new(PlayerState::Playing));
        let volume = Arc::new(Mutex::new(1.0));

        let state_clone = Arc::clone(&state);
        let volume_clone = Arc::clone(&volume);

        // Ce thread écoute les messages en arrière-plan sans bloquer le reste du programme
        thread::spawn(move || {
            while let Ok(command) = receiver.recv() {
                match command {
                    PlayerCommand::Play => {
                        *state_clone.lock().unwrap() = PlayerState::Playing;
                    }
                    PlayerCommand::Pause => {
                        *state_clone.lock().unwrap() = PlayerState::Paused;
                    }
                    PlayerCommand::Stop => {
                        *state_clone.lock().unwrap() = PlayerState::Stopped;
                    }
                    PlayerCommand::SetVolume(vol) => {
                        *volume_clone.lock().unwrap() = vol;
                    }
                }
            }
        });

        Self { command_sender: sender, state, volume }
    }

    pub fn play(&self) {
        let _ = self.command_sender.send(PlayerCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.command_sender.send(PlayerCommand::Pause);
    }

    pub fn stop(&self) {
        let _ = self.command_sender.send(PlayerCommand::Stop);
    }

    pub fn set_volume(&self, volume: f32) {
        let _ = self.command_sender.send(PlayerCommand::SetVolume(volume));
    }

    pub fn get_state(&self) -> PlayerState {
        self.state.lock().unwrap().clone()
    }

    pub fn get_volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_pipeline_commands() {
        let pipeline = Pipeline::new(); 

        // Vérifier l'état initial (Playing par défaut dans ton code)
        assert_eq!(pipeline.get_state(), PlayerState::Playing); 

        // Envoyer Pause et attendre un court instant le traitement du thread
        pipeline.pause(); //
        thread::sleep(Duration::from_millis(50)); 
        assert_eq!(pipeline.get_state(), PlayerState::Paused); 

        // Changer le volume
        pipeline.set_volume(0.5); 
        thread::sleep(Duration::from_millis(50));
        assert_eq!(pipeline.get_volume(), 0.5); 

        // Stopper
        pipeline.stop(); 
        thread::sleep(Duration::from_millis(50));
        assert_eq!(pipeline.get_state(), PlayerState::Stopped); 
    }
}