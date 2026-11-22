use symphonia::core::{
    codecs::{CodecParameters, audio::AudioCodecId},
    errors::Error as SymphoniaError,
    formats::FormatReader,
    io::MediaSourceStream,
};

use crate::media_probe::{
    audio_metadata::{AudioStreamMetadata, extract_audio_metadata},
    errors::{AudioEncodingError, AudioFormatError, AudioStreamError},
    prober::AudioProber,
    reader::{AudioTrackInfo, AudioTrackSelection, ProbedAudioReader},
};

pub struct IdentifiedAudioStream<'a> {
    pub classification: AudioStreamClassification,
    pub metadata: AudioStreamMetadata,
    pub track_id: u32,
    pub reader: Box<dyn FormatReader + 'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AudioStreamClassification {
    pub format: String,
    pub encoding: AudioCodecId,
}

/// @relation(APLLSR-001, scope=function)
pub(crate) fn identify_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
    selection: AudioTrackSelection,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
    let probed = prober
        .probe_stream(media_source_stream)
        .map_err(|error| map_probe_error(error))?;

    let identified_audio_track =
        identify_audio_track(probed.probed_reader.as_ref(), selection)?;

    Ok(IdentifiedAudioStream {
        classification: identified_audio_track.classification,
        metadata: identified_audio_track.metadata,
        track_id: identified_audio_track.track_info.track.id,
        reader: probed.reader,
    })
}

pub fn map_probe_error(error: SymphoniaError) -> AudioStreamError {
    match error {
        // @relation(APLLSR-005, scope=line)
        SymphoniaError::Unsupported(message) => AudioStreamError::Format(
            AudioFormatError::Unknown(message.to_string()),
        ),
        SymphoniaError::DecodeError(message) => {
            // @relation(APLLSR-007, scope=line)
            AudioStreamError::Format(AudioFormatError::Invalid(
                message.to_string(),
            ))
        }

        SymphoniaError::IoError(error) => AudioStreamError::Io(error),
        other => AudioStreamError::Symphonia(other),
    }
}

pub fn identify_audio_track(
    probed_reader: &dyn ProbedAudioReader,
    selection: AudioTrackSelection,
) -> Result<IdentifiedAudioTrack<'_>, AudioStreamError> {
    let format = probed_reader.format_short_name().to_owned();
    let track_info = probed_reader.select_audio_track(selection)?;

    let metadata = extract_audio_metadata(track_info.track)?;

    let classification = extract_classification(
        format,
        track_info.track.codec_params.as_ref(),
    )?;

    Ok(IdentifiedAudioTrack {
        classification,
        metadata,
        track_info,
    })
}

pub struct IdentifiedAudioTrack<'a> {
    pub classification: AudioStreamClassification,
    pub metadata: AudioStreamMetadata,
    pub track_info: AudioTrackInfo<'a>,
}

fn extract_classification(
    format: String,
    codec_params: Option<&CodecParameters>,
) -> Result<AudioStreamClassification, AudioStreamError> {
    // @relation(APLLSR-006, scope=line)
    let params = codec_params.ok_or(AudioStreamError::Encoding(
        AudioEncodingError::Unknown(
            "Audio codec parameters were not provided".to_owned(),
        ),
    ))?;

    let audio_params = match params {
        CodecParameters::Audio(audio) => audio,
        // @relation(APLLSR-003, scope=line)
        _ => {
            return Err(AudioStreamError::Format(
                AudioFormatError::Unsupported(
                    "Codec parameters do not describe an audio stream"
                        .to_owned(),
                ),
            ));
        }
    };

    Ok(AudioStreamClassification {
        format,
        encoding: audio_params.codec,
    })
}
