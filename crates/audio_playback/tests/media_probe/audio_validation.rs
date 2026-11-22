use audio_playback::media_probe::{
    audio_classification::AudioStreamClassification,
    audio_validation::{
        validate_audio_encoding, validate_audio_format,
        validate_format_encoding_combination, verify_format_supported,
    },
    errors::{
        AudioEncodingError, AudioFormatAndEncodingError, AudioFormatError,
        AudioStreamError,
    },
};
use symphonia::core::codecs::audio::{
    CODEC_ID_NULL_AUDIO,
    well_known::{CODEC_ID_FLAC, CODEC_ID_WAVPACK},
};

/// @relation(APLLSR-002-T01, scope=function)
#[test]
fn accepts_flac_format() {
    assert!(validate_audio_format("flac").is_ok());
}

/// @relation(APLLSR-002-T02, scope=function)
#[test]
fn accepts_flac_and_encoding_combination() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    assert!(validate_audio_encoding(classification.encoding).is_ok());
}

/// @relation(APLLSR-002-T03, scope=function)
#[test]
fn accepts_flac_format_and_encoding_combination() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    assert!(validate_format_encoding_combination(&classification).is_ok());
}

/// @relation(APLLSR-003-T01, scope=function)
#[test]
fn rejects_unsupported_audio_formats() {
    for format in ["mp3", "wav", "ogg", "aiff", "unknown"] {
        let result = validate_audio_format(format);

        assert!(matches!(
            result,
            Err(AudioStreamError::Format(AudioFormatError::Unsupported(_)))
        ));
    }
}

/// @relation(APLLSR-003-T01, scope=function)
#[test]
fn rejects_empty_format_name() {
    let result = validate_audio_format("");

    assert!(matches!(
        result,
        Err(AudioStreamError::Format(AudioFormatError::Unsupported(_)))
    ));
}

/// @relation(APLLSR-006-T03, scope=function)
#[test]
fn rejects_null_codec_identifier_with_valid_format() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_NULL_AUDIO,
    };

    let result = verify_format_supported(&classification);
    println!("result: {:?}", result);

    assert!(matches!(
        result,
        Err(AudioStreamError::Encoding(AudioEncodingError::Unknown(_)))
    ));
}

/// @relation(APLLSR-006-T03, scope=function)
#[test]
fn rejects_null_codec_identifier_with_unsupported_format() {
    let classification = AudioStreamClassification {
        format: "mka".to_owned(),
        encoding: CODEC_ID_NULL_AUDIO,
    };

    let result = verify_format_supported(&classification);
    println!("result: {:?}", result);

    assert!(matches!(
        result,
        Err(AudioStreamError::FormatAndEncoding(
            AudioFormatAndEncodingError {
                format_error: AudioFormatError::Unsupported(_),
                encoding_error: AudioEncodingError::Unknown(_),
            }
        )),
    ));
}

/// @relation(APLLSR-002-T04, scope=function)
#[test]
fn unsupported_format_and_unsupported_encoding() {
    let classification = AudioStreamClassification {
        format: "mka".to_owned(),
        encoding: CODEC_ID_WAVPACK,
    };

    let result = verify_format_supported(&classification);

    assert!(matches!(
        result,
        Err(AudioStreamError::FormatAndEncoding(
            AudioFormatAndEncodingError {
                format_error: AudioFormatError::Unsupported(_),
                encoding_error: AudioEncodingError::Unsupported(_),
            }
        )),
    ));
}

/// @relation(APLLSR-004-T01, scope=function)
#[test]
fn unsupported_encoding_is_returned_for_supported_format() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_WAVPACK,
    };

    let result = verify_format_supported(&classification);

    assert!(matches!(
        result,
        Err(AudioStreamError::Encoding(AudioEncodingError::Unsupported(
            _
        ))),
    ));
}
