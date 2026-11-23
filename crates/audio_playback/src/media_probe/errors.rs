use symphonia::core::errors::Error as SymphoniaError;

use crate::media_probe::reader::AudioTrackSelection;

#[derive(Debug, thiserror::Error)]
pub enum AudioStreamError {
    #[error("Audio format error: {0}")]
    Format(#[from] AudioFormatError),

    #[error("Audio encoding error: {0}")]
    Encoding(#[from] AudioEncodingError),

    #[error("Audio format and encoding error: {0}")]
    FormatAndEncoding(#[from] AudioFormatAndEncodingError),

    // @relation(APLLSR-009, scope=line)
    #[error("No audio track was found")]
    NoAudioTrack,

    #[error("Audio track not found: {0:?}")]
    AudioTrackNotFound(AudioTrackSelection),

    #[error("Audio metadata error: {0}")]
    Metadata(#[from] AudioStreamMetadataError),

    #[error("Audio stream I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symphonia error: {0}")]
    Symphonia(#[from] SymphoniaError),
}

#[derive(Debug, thiserror::Error)]
#[error("Format: {format_error}; encoding: {encoding_error}")]
pub struct AudioFormatAndEncodingError {
    pub format_error: AudioFormatError,
    pub encoding_error: AudioEncodingError,
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

#[derive(Debug, thiserror::Error)]
pub enum AudioStreamMetadataError {
    #[error("{0}")]
    InvalidChannels(ChannelError),

    #[error("{0}")]
    InvalidSampleRate(SampleRateError),

    #[error("{0}")]
    InvalidBitsPerSample(BitsPerSampleError),
}

#[derive(Debug, thiserror::Error)]
pub enum ChannelError {
    #[error("Too many channels")]
    TooMany,

    #[error("Too few channels")]
    TooFew,

    #[error("No channels found")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum SampleRateError {
    #[error("Sample Rate Too High")]
    TooHigh,

    #[error("Sample rate too low")]
    TooLow,

    #[error("Unknown sample rate")]
    Unknown,

    #[error("No sample rate found")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum BitsPerSampleError {
    #[error("Bits per sample is too high")]
    TooHigh,

    #[error("Bits per sample is too low")]
    TooLow,

    #[error("No bits per sample found")]
    NotFound,
}
