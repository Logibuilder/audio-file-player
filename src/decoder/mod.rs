use anyhow::{Result, anyhow};
use symphonia::core::{
    audio::SampleBuffer,
    codecs::DecoderOptions,
    formats::FormatOptions,
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
};
use std::fs::File;
use symphonia::core::{units::Time, formats::{SeekTo, SeekMode}};


/// Gère l'ouverture et le décodage des fichiers audio via `symphonia`.
pub struct Decoder {
    format: Box<dyn symphonia::core::formats::FormatReader>,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    sample_rate: u32,
    channels: u16,
}


impl Decoder {

    /// Ouvre un fichier audio à partir d'un chemin et initialise le décodeur approprié.
    pub fn open(path: &str) -> Result<Self> {

        let src = File::open(path)?;

        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let analyzer = symphonia::default::get_probe().format(
            &Hint::new(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let format = analyzer.format;

        let pist = format.default_track().ok_or_else(|| anyhow!("No default track found"))?;

        let frequence = pist.codec_params.sample_rate.unwrap_or(44100);

        let channels = pist.codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);
        
        let decoder = symphonia::default::get_codecs().make(&pist.codec_params, &DecoderOptions::default(),)?;

        Ok(Self {format, decoder, sample_rate: frequence, channels})

    }


    /// Lit le prochain paquet audio et le convertit en échantillons de type `f32` entrelacés.
    /// Retourne `None` si la fin du fichier est atteinte.
    pub fn next_chunk(&mut self) -> Option<Vec<f32>> {


        let packet = self.format.next_packet().ok()?;

        let decoded_data = self.decoder.decode(&packet).ok()?;

        let spec = *decoded_data.spec();

        let capacity = decoded_data.capacity() as usize;

        let mut buffer = SampleBuffer::<f32>::new(capacity as u64, spec);

        buffer.copy_interleaved_ref(decoded_data);

        Some(buffer.samples().to_vec())
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }


    /// Réinitialise la position du décodeur au début du fichier (0s).
    pub fn reset(&mut self) -> Result<()> {
        let seek_to = SeekTo::Time {
            time: Time::new(0, 0.0),
            track_id: None, 
        };
        
        self.format.seek(SeekMode::Accurate, seek_to)
            .map_err(|e| anyhow!("Erreur lors du retour au début : {}", e))?;
            
        // Très important : vider les buffers internes du décodeur
        self.decoder.reset();
        Ok(())
    }

}