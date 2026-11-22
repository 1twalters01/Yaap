use std::time::Duration;

use symphonia::core::{codecs::CodecParameters, formats::Track};

use crate::media_probe::errors::{AudioFormatError, AudioStreamError};

#[derive(Debug, Default)]
pub struct AudioStreamMetadata {
    pub channels: Option<usize>,
    pub sample_rate: Option<u32>,
    pub bits_per_sample: Option<u32>,
    pub duration: Option<Duration>,
}

pub fn extract_audio_metadata(
    track: &Track,
) -> Result<AudioStreamMetadata, AudioStreamError> {
    match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(audio_params)) => {
            let channels = &audio_params
                .channels
                .as_ref()
                .map(|channels| channels.count());
            let sample_rate = audio_params.sample_rate;
            let bits_per_sample = audio_params.bits_per_sample;
            let duration = track
                .time_base
                .zip(track.duration)
                .and_then(|(time_base, duration)| {
                    time_base.calc_duration(duration)
                })
                .and_then(|time| {
                    let duration = std::time::Duration::try_from_secs_f64(
                        time.as_secs_f64(),
                    )
                    .ok()?;

                    Some(duration)
                });

            Ok(AudioStreamMetadata {
                channels: *channels,
                sample_rate,
                bits_per_sample,
                duration,
            })
        }
        None => Ok(AudioStreamMetadata::default()),
        _ => {
            // @relation(APLLSR-003, scope=line)
            return Err(AudioStreamError::Format(
                AudioFormatError::Unsupported(
                    "Codec parameters do not describe an audio stream"
                        .to_owned(),
                ),
            ));
        }
    }
}
