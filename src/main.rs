mod decoder;
mod output;
mod pipeline;
use std::sync::{Arc, Mutex};
use std::io::{self, Write}; // Pour l'interaction console
use decoder::Decoder;
use output::cpal_output::CpalOutput;
use output::output::Output;
use crate::pipeline::Pipeline;

fn main() -> anyhow::Result<()> {
    // --- Phase de décodage ---
    let decoder = Arc::new(Mutex::new(Decoder::open("src/assets/audio2_test.mp3")?));
   
    // --- Phase d'initialisation ---
    let pipeline = Arc::new(Pipeline::new());
    let mut output_audio = CpalOutput::new(Arc::clone(&pipeline));

    
    // Lancement du flux audio
    output_audio.decoder = Some(Arc::clone(&decoder));
    output_audio.play(Vec::new()).map_err(|e| anyhow::anyhow!(e))?;
    // --- Boucle d'interaction ---
    println!("\n--- Commandes du Lecteur ---");
    println!("[p] Pause | [l] Lecture | [s] Stop | [v] Volume (ex: v 0.5) | [q] Quitter");

    loop {
        print!("> ");
        io::stdout().flush()?; // Force l'affichage immédiat du curseur
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let cmd = input.trim().to_lowercase();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
            ["p"] => {
                pipeline.pause(); // Utilise le Pipeline pour mettre en pause
                println!("Musique mise en pause.");
            },
            ["l"] => {
                pipeline.play(); // Reprend la lecture via le Pipeline
                println!("Reprise de la lecture.");
            },
            ["s"] => {
                pipeline.stop(); // Arrête et remet l'index à zéro
                println!("Lecture arrêtée.");
            },
            ["v", val] => {
                if let Ok(v) = val.parse::<f32>() {
                    pipeline.set_volume(v); // Change le volume dynamiquement
                    println!("Volume réglé à {}%", v * 100.0);
                }
            },
            ["q"] => {
                println!("Fermeture du lecteur.");
                break; // Sort de la boucle et termine le programme
            },
            _ => println!("Commande inconnue. Utilisez p, l, s, v <valeur> ou q."),
        }
    }

    Ok(())
}