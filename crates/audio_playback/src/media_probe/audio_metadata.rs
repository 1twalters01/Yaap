use std::time::Duration;

use symphonia::core::codecs::CodecParameters;

use crate::media_probe::errors::{AudioFormatError, AudioStreamError};

#[derive(Debug, Default)]
pub struct AudioStreamMetadata {
    pub channels: Option<u16>,
    pub sample_rate: Option<u32>,
    pub bits_per_sample: Option<u32>,
    pub duration: Option<Duration>,
}

pub fn extract_audio_metadata(codec_params: &CodecParameters) -> Result<AudioStreamMetadata, AudioStreamError> {
    match codec_params {
        CodecParameters::Audio(audio_params) => {
            let channels = &audio_params.channels;
            let sample_rate = audio_params.sample_rate;
            let bits_per_sample = audio_params.bits_per_sample;

            Ok(AudioStreamMetadata::default())
        },
        // @relation(APLLSR-003, scope=line)
        _ => {
            return Err(AudioStreamError::Format(
                AudioFormatError::Unsupported(
                    "Codec parameters do not describe an audio stream"
                        .to_owned(),
                ),
            ));
        }
    }
}
