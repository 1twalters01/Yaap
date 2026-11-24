use audio_playback::media_probe::{
    audio_classification::AudioStreamClassification,
    audio_metadata::extract_audio_metadata,
    audio_validation::{
        validate_bits_per_sample, validate_channels, validate_sample_rate,
    },
    errors::{
        AudioStreamError, AudioStreamMetadataError, BitsPerSampleError,
        ChannelError, SampleRateError,
    },
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

// Channel Count
fn validate_flac_channel_count(
    channels: Option<Channels>,
) -> Result<(), AudioStreamError> {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params = flac_codec_params(channels, Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    validate_channels(&classification, metadata.channels)
}

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_valid() {
    let valid_channels = Some(Channels::Discrete(6));
    let result = validate_flac_channel_count(valid_channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T02, scope=function)
#[test]
fn flac_channel_count_is_upper_boundary() {
    let upper_boundary_channels = Some(Channels::Discrete(8));
    let result = validate_flac_channel_count(upper_boundary_channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T03, scope=function)
#[test]
fn flac_channel_count_is_lower_boundary() {
    let lower_boundary_channels = Some(Channels::Discrete(1));
    let result = validate_flac_channel_count(lower_boundary_channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T04, scope=function)
#[test]
fn flac_channel_count_is_too_high() {
    let too_many_channels = Some(Channels::Discrete(9));
    let result = validate_flac_channel_count(too_many_channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooMany)
        )),
    ));
}

/// @relation(APLLSR-013-T05, scope=function)
#[test]
fn flac_channel_count_is_way_too_high() {
    let too_many_channels = Some(Channels::Discrete(9_200));
    let result = validate_flac_channel_count(too_many_channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooMany)
        )),
    ));
}

/// @relation(APLLSR-013-T06, scope=function)
#[test]
fn flac_channel_count_is_too_low() {
    let too_few_channels = Some(Channels::Discrete(0));
    let result = validate_flac_channel_count(too_few_channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::TooFew)
        )),
    ));
}

/// @relation(APLLSR-013-T07, scope=function)
#[test]
fn flac_channel_count_is_none() {
    let no_channels = None;
    let result = validate_flac_channel_count(no_channels);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidChannels(ChannelError::NotFound)
        )),
    ));
}

// Sample Rate
fn validate_flac_sample_rate(
    sample_rate: Option<u32>,
) -> Result<(), AudioStreamError> {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), sample_rate, Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    validate_sample_rate(&classification, metadata.sample_rate)
}

/// @relation(APLLSR-015-T01, scope=function)
#[test]
fn flac_sample_rate_is_valid() {
    let valid_sample_rate = Some(55_350);
    let result = validate_flac_sample_rate(valid_sample_rate);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-015-T02, scope=function)
#[test]
fn flac_sample_rate_is_upper_boundary() {
    let upper_boundary_sample_rate = Some(655_350);
    let result = validate_flac_sample_rate(upper_boundary_sample_rate);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-015-T03, scope=function)
#[test]
fn flac_sample_rate_is_lower_boundary() {
    let lower_boundary_sample_rate = Some(1);
    let result = validate_flac_sample_rate(lower_boundary_sample_rate);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-015-T04, scope=function)
#[test]
fn flac_sample_rate_is_too_high() {
    let too_high_sample_rate = Some(655_351);
    let result = validate_flac_sample_rate(too_high_sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::TooHigh
            )
        )),
    ));
}

/// @relation(APLLSR-015-T05, scope=function)
#[test]
fn flac_sample_rate_is_way_too_high() {
    let too_high_sample_rate = Some(1_655_351);
    let result = validate_flac_sample_rate(too_high_sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::TooHigh
            )
        )),
    ));
}

/// @relation(APLLSR-015-T06, scope=function)
#[test]
fn flac_sample_rate_is_too_low() {
    let too_low_sample_rate = Some(0);
    let result = validate_flac_sample_rate(too_low_sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::Unknown
            )
        )),
    ));
}

/// @relation(APLLSR-015-T07, scope=function)
#[test]
fn flac_sample_rate_is_none() {
    let no_sample_rate = None;
    let result = validate_flac_sample_rate(no_sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::NotFound
            )
        )),
    ));
}

// Bits per sample
fn validate_flac_bits_per_sample(
    bits_per_sample: Option<u32>,
) -> Result<(), AudioStreamError> {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params = flac_codec_params(
        Some(Channels::Discrete(6)),
        Some(44_100),
        bits_per_sample,
    );
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    validate_bits_per_sample(&classification, metadata.bits_per_sample)
}

#[test]
fn flac_bits_per_sample_is_valid() {
    let valid_bits_per_sample = Some(16);
    let result = validate_flac_bits_per_sample(valid_bits_per_sample);

    assert!(matches!(result, Ok(()),));
}

#[test]
fn flac_bits_per_sample_is_upper_boundary() {
    let upper_boundary_bits_per_sample = Some(32);
    let result = validate_flac_bits_per_sample(upper_boundary_bits_per_sample);

    assert!(matches!(result, Ok(()),));
}

#[test]
fn flac_bits_per_sample_is_lower_boundary() {
    let lower_boundary_bits_per_sample = Some(32);
    let result = validate_flac_bits_per_sample(lower_boundary_bits_per_sample);

    assert!(matches!(result, Ok(()),));
}

#[test]
fn flac_bits_per_sample_is_too_high() {
    let too_high_bits_per_sample = Some(33);
    let result = validate_flac_bits_per_sample(too_high_bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooHigh
            )
        )),
    ));
}

#[test]
fn flac_bits_per_sample_is_way_too_high() {
    let too_high_bits_per_sample = Some(3_035);
    let result = validate_flac_bits_per_sample(too_high_bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooHigh
            )
        )),
    ));
}

#[test]
fn flac_bits_per_sample_is_too_low() {
    let too_low_bits_per_sample = Some(3);
    let result = validate_flac_bits_per_sample(too_low_bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooLow
            )
        )),
    ));
}

#[test]
fn flac_bits_per_sample_is_zero() {
    let zero_bits_per_sample = Some(3);
    let result = validate_flac_bits_per_sample(zero_bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::TooLow
            )
        )),
    ));
}

#[test]
fn flac_bits_per_sample_is_none() {
    let no_bits_per_sample = None;
    let result = validate_flac_bits_per_sample(no_bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::NotFound
            )
        )),
    ));
}
