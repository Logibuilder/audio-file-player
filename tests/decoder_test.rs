use audio_file_player::decoder::Decoder;

#[test]
fn open_invalid_path_returns_error() {
    assert!(Decoder::open("/chemin/inexistant.mp3").is_err());
}

#[test]
#[ignore = "nécessite TEST_AUDIO_FILE"]
fn open_and_read_valid_file() {
    let path = std::env::var("TEST_AUDIO_FILE").unwrap();
    let mut decoder = Decoder::open(&path).unwrap();
    assert!(decoder.sample_rate() > 0);
    let chunk = decoder.next_chunk().unwrap();
    assert!(!chunk.is_empty());
    decoder.reset().unwrap();
    assert!(decoder.next_chunk().is_some());
}