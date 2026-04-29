use crate::output::output::Output;
use std::io::{self, Write};

pub fn print_help() {
    println!("\n--- Commandes du Lecteur ---");
    println!("[p] Pause | [l] Lecture | [s] Stop | [v] Volume (ex: v 0.5) | [q] Quitter");
}


pub fn run_cli<T : Output>(mut output_audio : T) -> anyhow::Result<()> {
    print_help();

    loop {
        print!("> ");
        io::stdout().flush()?; // Force l'affichage immédiat du curseur
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let cmd = input.trim().to_lowercase();
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
            ["p"] => {
                let _ = output_audio.pause(); // Utilise output_audio !
                println!("Musique mise en pause.");
            },
            ["l"] => {
                let _ = output_audio.replay(); // Utilise output_audio !
                println!("Reprise de la lecture.");
            },
            ["s"] => {
                let _ = output_audio.stop(); // C'est ICI que dec.reset() va s'exécuter !
                println!("Lecture arrêtée (retour au début).");
            },
            ["v", val] => {
                if let Ok(v) = val.parse::<f32>() {
                    output_audio.set_volume(v); // Utilise output_audio !
                    println!("Volume réglé à {}%", v * 100.0);
                }
            },
            ["q"] => {
                println!("Fermeture du lecteur.");
                break;
            },
            _ => println!("Commande inconnue. Utilisez p, l, s, v <valeur> ou q."),
        }
    }
    Ok(())
}