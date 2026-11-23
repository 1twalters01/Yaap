use std::num::NonZero;

use symphonia::core::codecs::audio::{
    AudioCodecId, CODEC_ID_NULL_AUDIO, well_known::CODEC_ID_FLAC,
};

use crate::media_probe::{
    audio_classification::AudioStreamClassification,
    errors::{
        AudioEncodingError, AudioFormatAndEncodingError, AudioFormatError,
        AudioStreamError, AudioStreamMetadataError, BitsPerSampleError,
        ChannelError, SampleRateError,
    },
};

pub fn verify_format_supported(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    let format_result = validate_audio_format(&classification.format);
    let encoding_result = validate_audio_encoding(classification.encoding);

    match (format_result, encoding_result) {
        (Ok(()), Ok(())) => {
            validate_format_encoding_combination(classification)
        }
        (
            Err(AudioStreamError::Format(format)),
            Err(AudioStreamError::Encoding(encoding)),
        ) => Err(AudioStreamError::FormatAndEncoding(
            AudioFormatAndEncodingError {
                format_error: format,
                encoding_error: encoding,
            },
        )),
        (Err(error), _) | (_, Err(error)) => Err(error),
    }
}

pub fn validate_audio_format(format: &str) -> Result<(), AudioStreamError> {
    match format {
        "flac" => Ok(()),
        // @relation(APLLSR-003, scope=line)
        other => Err(AudioStreamError::Format(AudioFormatError::Unsupported(
            format!("Audio format '{other}' is not supported"),
        ))),
    }
}

pub fn validate_audio_encoding(
    encoding: AudioCodecId,
) -> Result<(), AudioStreamError> {
    match encoding {
        CODEC_ID_NULL_AUDIO => {
            // @relation(APLLSR-006, scope=line)
            Err(AudioStreamError::Encoding(AudioEncodingError::Unknown(
                "Audio codec identifier is null".to_owned(),
            )))
        }

        // @relation(APLLSR-010, scope=line)
        CODEC_ID_FLAC => Ok(()),

        // @relation(APLLSR-004, scope=line)
        other => {
            Err(AudioStreamError::Encoding(AudioEncodingError::Unsupported(
                format!("Audio encoding '{other}' is not supported"),
            )))
        }
    }
}

pub fn validate_format_encoding_combination(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    match (classification.format.as_str(), classification.encoding) {
        // @relation(APLLSR-011, scope=line)
        ("flac", CODEC_ID_FLAC) => Ok(()),

        // @relation(APLLSR-004, scope=line)
        (format, encoding) => Err(AudioStreamError::Encoding(
            AudioEncodingError::Unsupported(format!(
                "Audio encoding '{encoding}' is not supported in audio format '{format}'"
            )),
        )),
    }
}

pub fn validate_channels(
    classification: &AudioStreamClassification,
    channels: Option<usize>,
) -> Result<(), AudioStreamError> {
    let channels = channels.ok_or(AudioStreamError::Metadata(
        AudioStreamMetadataError::InvalidChannels(ChannelError::NotFound),
    ))?;

    let channels =
        NonZero::new(channels).ok_or(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooFew),
        ))?;

    match classification.encoding {
        CODEC_ID_FLAC => validate_flac_channels(channels),
        _ => Ok(()),
    }
}

/// @relation(APLLSR-013, scope=function)
fn validate_flac_channels(
    channels: NonZero<usize>,
) -> Result<(), AudioStreamError> {
    if channels.get() <= 8 {
        Ok(())
    } else {
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooMany),
        ))
    }
}

pub fn validate_sample_rate(
    classification: &AudioStreamClassification,
    sample_rate: Option<u32>,
) -> Result<(), AudioStreamError> {
    let sample_rate = sample_rate.ok_or(AudioStreamError::Metadata(
        AudioStreamMetadataError::InvalidSampleRate(SampleRateError::NotFound),
    ))?;

    match classification.encoding {
        CODEC_ID_FLAC => validate_flac_sample_rate(sample_rate),
        _ => Ok(()),
    }
}

/// @relation(APLLSR-015, scope=function)
fn validate_flac_sample_rate(
    sample_rate: u32,
) -> Result<(), AudioStreamError> {
    match sample_rate {
        1..=655_350 => Ok(()),
        0 => Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::Unknown,
            ),
        )),
        655_351.. => Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::TooHigh,
            ),
        )),
    }
}

pub fn validate_bits_per_sample(
    classification: &AudioStreamClassification,
    bits_per_sample: Option<u32>,
) -> Result<(), AudioStreamError> {
    let bits_per_sample =
        bits_per_sample.ok_or(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::NotFound,
            ),
        ))?;

    match classification.encoding {
        CODEC_ID_FLAC => validate_flac_bits_per_sample(bits_per_sample),
        _ => Ok(()),
    }
}

/// @relation(APLLSR-017, scope=function)
fn validate_flac_bits_per_sample(
    bits_per_sample: u32,
) -> Result<(), AudioStreamError> {
    match bits_per_sample {
        4..=32 => Ok(()),
        33.. => Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooHigh,
            ),
        )),
        0..4 => Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooLow,
            ),
        )),
    }
}
