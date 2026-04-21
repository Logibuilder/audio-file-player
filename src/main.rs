mod decoder;
mod output; // Importe le dossier output
mod pipeline; // Importe le dossier pipeline
use std::sync::Arc;
use decoder::Decoder;
use output::cpal_output::CpalOutput; // Importe ta structure
use output::output::Output;        // Importe le Trait pour pouvoir utiliser .play()
use crate::pipeline::Pipeline; // Importe le Pipeline
fn main() -> anyhow::Result<()> {

    let mut decoder = Decoder::open("src/assets/test.wav")?;
    
    let mut musik = Vec::new();
    while let Some(mut morceau) = decoder.next_chunk() {
        musik.append(&mut morceau);
    }
    println!("Fichier décodé : {} échantillons prêts.", musik.len());

    let pipeline = Arc::new(Pipeline::new());
    let mut output_audio = CpalOutput::new(Arc::clone(&pipeline));

    
    output_audio.play(musik).map_err(|e| anyhow::anyhow!(e))?;

    println!("Lecture en cours... Appuyez sur Entrée pour arrêter.");
    
    let mut pause_input = String::new();
    std::io::stdin().read_line(&mut pause_input)?;

    Ok(())
}