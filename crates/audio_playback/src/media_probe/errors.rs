use std::fmt;

use symphonia::core::errors::Error as SymphoniaError;

use crate::media_probe::reader::AudioTrackSelection;

#[derive(Debug, thiserror::Error)]
pub enum AudioStreamError {
    #[error("Audio format error: {0}")]
    Format(AudioFormatError),

    #[error("Audio encoding error: {0}")]
    Encoding(AudioEncodingError),

    #[error("Audio format and encoding error: {0}")]
    FormatAndEncoding(AudioFormatAndEncodingError),

    // @relation(APLLSR-009, scope=line)
    #[error("No audio track was found")]
    NoAudioTrack,

    #[error("Audio track not found: {0:?}")]
    AudioTrackNotFound(AudioTrackSelection),

    #[error("Audio stream I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symphonia error: {0}")]
    Symphonia(#[from] SymphoniaError),
}

#[derive(Debug)]
pub struct AudioFormatAndEncodingError {
    pub format_error: AudioFormatError,
    pub encoding_error: AudioEncodingError,
}

impl fmt::Display for AudioFormatAndEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.format_error, self.encoding_error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AudioFormatError {
    #[error("Unknown audio format: {0}")]
    Unknown(String),

    #[error("Unsupported audio format: {0}")]
    Unsupported(String),

    #[error("Invalid audio format: {0}")]
    Invalid(String),
}

#[derive(Debug, thiserror::Error)]
pub enum AudioEncodingError {
    #[error("Unknown audio encoding: {0}")]
    Unknown(String),

    #[error("Unsupported audio encoding: {0}")]
    Unsupported(String),

    #[error("Invalid audio encoding: {0}")]
    Invalid(String),
}
