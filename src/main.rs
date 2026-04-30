//! # Audio File Player
//! 
//! Ce projet est un lecteur audio simple en ligne de commande écrit en Rust.
//! Il utilise `cpal` pour la sortie audio et `symphonia` pour le décodage.

mod decoder;
mod output;
mod pipeline;
mod ui;
use std::sync::{Arc, Mutex};
use decoder::Decoder;
use output::cpal_output::CpalOutput;
use output::output::Output;
use crate::pipeline::Pipeline;
use std::env;

fn main() -> anyhow::Result<()> {

    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <chemin_du_fichier_audio>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Lecture de : {}", file_path);

    // --- Phase de décodage ---
    let decoder = Arc::new(Mutex::new(Decoder::open(file_path)?));
   
    // --- Phase d'initialisation ---
    let pipeline = Arc::new(Pipeline::new());
    let mut output_audio = CpalOutput::new(Arc::clone(&pipeline));

    
    // Lancement du flux audio
    output_audio.decoder = Some(Arc::clone(&decoder));
    output_audio.play(Vec::new()).map_err(|e| anyhow::anyhow!(e))?;
    

    ui::run_cli(output_audio)?;
    Ok(())
}