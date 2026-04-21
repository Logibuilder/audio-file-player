mod decoder;
mod output; // Importe le dossier output

use decoder::Decoder;
use output::cpal_output::CpalOutput; // Importe ta structure
use output::output::Output;        // Importe le Trait pour pouvoir utiliser .play()

fn main() -> anyhow::Result<()> {


    let mut decoder = Decoder::open("src/assets/test.wav")?;
    
    let mut musik = Vec::new();
    while let Some(mut morceau) = decoder.next_chunk() {
        musik.append(&mut morceau);
    }
    println!("Fichier décodé : {} échantillons prêts.", musik.len());

    
    let mut output_audio = CpalOutput::new();

    
    output_audio.play(musik).map_err(|e| anyhow::anyhow!(e))?;

    println!("Lecture en cours... Appuyez sur Entrée pour arrêter.");
    
    let mut pause_input = String::new();
    std::io::stdin().read_line(&mut pause_input)?;

    Ok(())
}