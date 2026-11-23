use audio_playback::media_probe::{
    audio_classification::AudioStreamClassification, audio_metadata::extract_audio_metadata, audio_validation::{validate_bits_per_sample, validate_channels, validate_sample_rate}, errors::{
        AudioStreamError, AudioStreamMetadataError, BitsPerSampleError, ChannelError, SampleRateError,
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

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_valid() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_upper_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(8)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T01, scope=function)
#[test]
fn flac_channel_count_is_lower_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(1)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_channels(&classification, metadata.channels);

    assert!(matches!(result, Ok(()),));
}

/// @relation(APLLSR-013-T02, scope=function)
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

/// @relation(APLLSR-013-T02, scope=function)
#[test]
fn flac_channel_count_is_way_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params = flac_codec_params(
        Some(Channels::Discrete(9_200)),
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

/// @relation(APLLSR-013-T03, scope=function)
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

/// @relation(APLLSR-013-T04, scope=function)
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

// Sample Rate

#[test]
fn flac_sample_rate_is_valid() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_sample_rate_is_upper_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(655_350), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_sample_rate_is_lower_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(1), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_sample_rate_is_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(655_351), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::TooHigh
            )
        )),
    ));
}

#[test]
fn flac_sample_rate_is_way_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(1_655_351), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::TooHigh
            )
        )),
    ));
}

#[test]
fn flac_sample_rate_is_too_low() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(0), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidSampleRate(
                SampleRateError::Unknown
            )
        )),
    ));
}

#[test]
fn flac_sample_rate_is_none() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), None, Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_sample_rate(&classification, metadata.sample_rate);

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

#[test]
fn flac_bits_per_sample_is_valid() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(16));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_bits_per_sample_is_upper_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(32));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_bits_per_sample_is_lower_boundary() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(4));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

    assert!(matches!(
        result,
        Ok(()),
    ));
}

#[test]
fn flac_bits_per_sample_is_too_high() {
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(33));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

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
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(3_035));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

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
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(3));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

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
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), Some(0));
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

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
    let classification = AudioStreamClassification {
        format: "flac".to_owned(),
        encoding: CODEC_ID_FLAC,
    };

    let codec_params =
        flac_codec_params(Some(Channels::Discrete(6)), Some(55_350), None);
    let track = stub_track(Some(codec_params));

    let metadata = extract_audio_metadata(&track).unwrap();

    let result = validate_bits_per_sample(&classification, metadata.bits_per_sample);

    assert!(matches!(
        result,
        Err(AudioStreamError::Metadata(
            AudioStreamMetadataError::InvalidBitsPerSample(
                BitsPerSampleError::NotFound
            )
        )),
    ));
}
