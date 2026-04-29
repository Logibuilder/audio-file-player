mod decoder;
mod output;
mod pipeline;
mod ui;
use std::sync::{Arc, Mutex};
use decoder::Decoder;
use output::cpal_output::CpalOutput;
use output::output::Output;
use crate::pipeline::Pipeline;

fn main() -> anyhow::Result<()> {
    // --- Phase de décodage ---
    let decoder = Arc::new(Mutex::new(Decoder::open("src/assets/Juggernaut.mp3")?));
   
    // --- Phase d'initialisation ---
    let pipeline = Arc::new(Pipeline::new());
    let mut output_audio = CpalOutput::new(Arc::clone(&pipeline));

    
    // Lancement du flux audio
    output_audio.decoder = Some(Arc::clone(&decoder));
    output_audio.play(Vec::new()).map_err(|e| anyhow::anyhow!(e))?;
    

    ui::run_cli(output_audio)?;
    Ok(())
}