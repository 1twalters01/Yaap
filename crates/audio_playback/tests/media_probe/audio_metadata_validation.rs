use audio_playback::media_probe::{
    audio_classification::AudioStreamClassification,
    audio_metadata::extract_audio_metadata,
    audio_validation::validate_channels,
    errors::{AudioStreamError, AudioStreamMetadataError, ChannelError},
};
use symphonia::core::{
    audio::Channels,
    codecs::{
        CodecParameters,
        audio::{AudioCodecParameters, well_known::CODEC_ID_FLAC},
    },
    formats::{Track, TrackFlags},
};

fn stub_track(codec_params: Option<CodecParameters>) -> Track {
    Track {
        id: 1,
        codec_params,
        language: None,
        time_base: None,
        num_frames: None,
        duration: None,
        start_ts: 0.into(),
        delay: None,
        padding: None,
        flags: TrackFlags::empty(),
    }
}

fn flac_audio_codec_params(
    channels: Option<Channels>,
    sample_rate: Option<u32>,
    bits_per_sample: Option<u32>,
) -> AudioCodecParameters {
    let mut params = AudioCodecParameters::new();
    params.for_codec(CODEC_ID_FLAC);
    params.channels = channels;
    params.sample_rate = sample_rate;
    params.bits_per_sample = bits_per_sample;
    params
}

fn flac_codec_params(
    channels: Option<Channels>,
    sample_rate: Option<u32>,
    bits_per_sample: Option<u32>,
) -> CodecParameters {
    CodecParameters::Audio(flac_audio_codec_params(
        channels,
        sample_rate,
        bits_per_sample,
    ))
}

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(9)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooMany)
        )),
    ));
}

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_way_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params = flac_codec_params(
        Some(Channels::Discrete(200)),
        Some(44_100),
        Some(16),
    );
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooMany)
        )),
    ));
}

/// @relation(APLLSR-013-T02, scope=function)
#[test]
fn flac_channel_count_is_too_low() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(0)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooFew)
        )),
    ));
}

/// @relation(APLLSR-013-T03, scope=function)
#[test]
fn flac_channel_count_is_none() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params = flac_codec_params(None, Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::NotFound)
        )),
    ));
}
