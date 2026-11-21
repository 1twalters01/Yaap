use std::fs::File;

use audio_playback::media_probe::{errors::{AudioFormatError, AudioStreamError}, identify_supported_audio_stream, prober::SymphoniaProber, reader::AudioTrackSelection};
use symphonia::core::{
    codecs::audio::well_known::CODEC_ID_FLAC, io::MediaSourceStream,
};

/// @relation(APLLSR-010-T01, scope=function)
/// @relation(APLLSR-011-T01, scope=function)
#[test]
fn identifies_valid_flac_file() {
    let file =
        File::open("/data/Programming/Tests/data/music/flac/music.flac")
            .unwrap();

    let source = MediaSourceStream::new(Box::new(file), Default::default());

    let prober = SymphoniaProber::default();
    let audio_track_selection = AudioTrackSelection::First;

    let stream = identify_supported_audio_stream(
        source,
        &prober,
        audio_track_selection,
    )
    .unwrap();

    assert_eq!(stream.classification.format, "flac");
    assert_eq!(stream.classification.encoding, CODEC_ID_FLAC);
}

/// @relation(APLLSR-005-T01, scope=function)
#[test]
fn returns_unknown_format_for_unrecognised_input() {
    let file =
        File::open("/data/Programming/Tests/data/music/lrc/04 Mala mia.lrc")
            .unwrap();

    let source = MediaSourceStream::new(Box::new(file), Default::default());

    let prober = SymphoniaProber::default();
    let audio_track_selection = AudioTrackSelection::First;

    let result = identify_supported_audio_stream(
        source,
        &prober,
        audio_track_selection,
    );

    assert!(matches!(
        result,
        Err(AudioStreamError::Format(AudioFormatError::Unknown(_))),
    ));
}

/// @relation(APLLSR-011-T02, scope=function)
#[test]
fn identifies_flac_independently_of_filename_extension() {
    for path in [
        "/data/Programming/Tests/data/music/flac/music.flac",
        "/data/Programming/Tests/data/music/flac/music.lrc",
        "/data/Programming/Tests/data/music/flac/music.mp4",
    ] {
        let file = File::open(path).unwrap();

        let source =
            MediaSourceStream::new(Box::new(file), Default::default());

        let prober = SymphoniaProber::default();
        let audio_track_selection = AudioTrackSelection::First;

        let stream = identify_supported_audio_stream(
            source,
            &prober,
            audio_track_selection,
        )
        .unwrap();

        assert_eq!(
            stream.classification.format, "flac",
            "Failed to identify FLAC data in {path}"
        );
    }
}

/// @relation(APLLSR-011-T03, scope=function)
#[test]
fn identifies_flac_without_filename_extension() {
    let path = "/data/Programming/Tests/data/music/flac/music";
    let file = File::open(path).unwrap();

    let source = MediaSourceStream::new(Box::new(file), Default::default());

    let prober = SymphoniaProber::default();
    let audio_track_selection = AudioTrackSelection::First;

    let stream = identify_supported_audio_stream(
        source,
        &prober,
        audio_track_selection,
    )
    .unwrap();

    assert_eq!(
        stream.classification.format, "flac",
        "Failed to identify FLAC data in {path}"
    );
}
