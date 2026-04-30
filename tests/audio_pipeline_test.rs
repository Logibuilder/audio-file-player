use audio_file_player::{
    output::{cpal_output::CpalOutput, output::Output},
    pipeline::{Pipeline, PlayerState},
};
use std::{sync::Arc, thread, time::Duration};

fn wait() {
    thread::sleep(Duration::from_millis(60));
}

fn make_output() -> (Arc<Pipeline>, CpalOutput) {
    let pipeline = Arc::new(Pipeline::new());
    let output = CpalOutput::new(Arc::clone(&pipeline));
    (pipeline, output)
}

#[test]
fn pipeline_state_cycle() {
    let pipeline = Pipeline::new();
    assert_eq!(pipeline.get_state(), PlayerState::Playing);

    pipeline.pause(); wait();
    assert_eq!(pipeline.get_state(), PlayerState::Paused);

    pipeline.play(); wait();
    assert_eq!(pipeline.get_state(), PlayerState::Playing);

    pipeline.stop(); wait();
    assert_eq!(pipeline.get_state(), PlayerState::Stopped);
}

#[test]
fn pipeline_volume() {
    let pipeline = Pipeline::new();
    pipeline.set_volume(0.5); wait();
    assert!((pipeline.get_volume() - 0.5).abs() < 1e-5);
}

#[test]
fn output_commands_propagate_to_pipeline() {
    let (pipeline, mut output) = make_output();

    output.pause().unwrap(); wait();
    assert_eq!(pipeline.get_state(), PlayerState::Paused);

    output.replay().unwrap(); wait();
    assert_eq!(pipeline.get_state(), PlayerState::Playing);

    output.set_volume(0.3); wait();
    assert!((pipeline.get_volume() - 0.3).abs() < 1e-5);
}