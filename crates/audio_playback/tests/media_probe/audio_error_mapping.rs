use std::io;

use audio_playback::media_probe::{AudioProperty, AudioPropertyType, AudioStreamError, map_probe_error};
use symphonia::core::errors::Error as SymphoniaError;

/// @relation(APLLSR-005-T01, scope=function)
#[test]
fn maps_unsupported_probe_error_to_unknown_format() {
    let result = map_probe_error(
        SymphoniaError::Unsupported("unrecognised format".into())
    );

    assert!(matches!(
        result,
        AudioStreamError::Unknown(AudioProperty {
            property: AudioPropertyType::Format,
            message: _,
        })
    ));
}

/// @relation(APLLSR-007-T01, scope=function)
#[test]
fn maps_decode_error_to_invalid_format() {
    let result = map_probe_error(
        SymphoniaError::DecodeError("invalid stream".into())
    );

    assert!(matches!(
        result,
        AudioStreamError::Invalid(AudioProperty{
            property: AudioPropertyType::Format,
            message: _
        })
    ));
}

/// @relation(APLLSR-007-T01, scope=function)
#[test]
fn preserves_io_error_kind_when_mapping_probe_error() {
    let result = map_probe_error(
        SymphoniaError::IoError(
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "access denied",
            )
        )
    );

    assert!(matches!(
        result,
        AudioStreamError::Io(error)
            if error.kind() == io::ErrorKind::PermissionDenied
    ));
}
