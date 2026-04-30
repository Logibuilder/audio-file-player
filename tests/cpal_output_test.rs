use audio_file_player::{
    output::{cpal_output::CpalOutput, output::Output},
    pipeline::Pipeline,
};
use std::sync::Arc;

#[test]
fn no_stream_before_play() {
    let pipeline = Arc::new(Pipeline::new());
    let output = CpalOutput::new(Arc::clone(&pipeline));
    assert!(output.stream.is_none());
}

#[test]
fn play_without_decoder_fails_cleanly() {
    let pipeline = Arc::new(Pipeline::new());
    let mut player = CpalOutput::new(Arc::clone(&pipeline));
    assert!(player.play(vec![]).is_err());
}