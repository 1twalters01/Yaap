use std::time::Duration;

use symphonia::core::{
    codecs::{CodecParameters, audio::AudioCodecParameters},
    formats::Track,
};

use crate::media_probe::errors::{AudioFormatError, AudioStreamError};

#[derive(Debug, Default)]
pub struct AudioStreamMetadata {
    pub channels: Option<usize>,
    pub sample_rate: Option<u32>,
    pub bits_per_sample: Option<u32>,
    pub duration: Option<Duration>,
}

impl AudioStreamMetadata {
    pub fn from_audio_params(
        audio_codec_params: &AudioCodecParameters,
        duration: Option<Duration>,
    ) -> Self {
        // @relation(APLLSR-012, scope=line)
        let channels = audio_codec_params
            .channels
            .as_ref()
            .map(|channels| channels.count());

        // @relation(APLLSR-014, scope=line)
        let sample_rate = audio_codec_params.sample_rate;

        // @relation(APLLSR-016, scope=line)
        let bits_per_sample = audio_codec_params.bits_per_sample;

        Self {
            channels,
            sample_rate,
            bits_per_sample,
            duration,
        }
    }
}

pub fn extract_audio_metadata(
    track: &Track,
) -> Result<AudioStreamMetadata, AudioStreamError> {
    match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(audio_codec_params)) => {
            let duration = match extract_sample_accurate_duration(
                track,
                audio_codec_params,
            ) {
                Some(duration) => Some(duration),
                // @relation(APLLSR-020, scope=line)
                None => extract_track_duration(track),
            };

            Ok(AudioStreamMetadata::from_audio_params(
                audio_codec_params,
                duration,
            ))
        }
        None => Ok(AudioStreamMetadata::default()),
        // @relation(APLLSR-003, scope=line)
        _ => Err(AudioStreamError::Format(AudioFormatError::Unsupported(
            "Codec parameters do not describe an audio stream".to_owned(),
        ))),
    }
}

fn extract_sample_accurate_duration(
    track: &Track,
    audio_codec_params: &AudioCodecParameters,
) -> Option<Duration> {
    let sample_rate = audio_codec_params.sample_rate? as u64;
    if sample_rate == 0 {
        return None;
    }

    let total_frames = track.num_frames?;
    if total_frames == 0 {
        return None;
    }

    // @relation(APLLSR-018, scope=line)
    let duration_nanoseconds =
        u128::from(total_frames) * 1_000_000_000 / u128::from(sample_rate);

    let seconds = u64::try_from(duration_nanoseconds / 1_000_000_000).ok()?;
    let nanoseconds =
        u32::try_from(duration_nanoseconds % 1_000_000_000).ok()?;

    Some(Duration::new(seconds, nanoseconds))
}

fn extract_track_duration(track: &Track) -> Option<Duration> {
    let time_base = track.time_base?;
    let duration_ticks = track.duration?;

    // @relation(APLLSR-019, scope=line)
    let time = time_base.calc_duration(duration_ticks)?;
    let (seconds, nanoseconds) = time.parts();
    let seconds = u64::try_from(seconds).ok()?;

    Some(Duration::new(seconds, nanoseconds))
}
