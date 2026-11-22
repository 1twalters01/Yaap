use symphonia::core::io::MediaSourceStream;

use crate::media_probe::{
    audio_classification::{IdentifiedAudioStream, identify_audio_stream},
    audio_validation::verify_format_supported,
    errors::AudioStreamError,
    prober::AudioProber,
    reader::AudioTrackSelection,
};

pub mod audio_classification;
pub mod audio_metadata;
pub mod audio_validation;
pub mod errors;
pub mod prober;
pub mod reader;

/// @relation(APLLSR-002, scope=function)
pub fn identify_supported_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
    selection: AudioTrackSelection,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
    let identified_audio_stream =
        identify_audio_stream(media_source_stream, prober, selection)?;
    verify_format_supported(&identified_audio_stream.classification)?;

    Ok(identified_audio_stream)
}
