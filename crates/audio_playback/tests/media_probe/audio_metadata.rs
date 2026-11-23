use std::{num::NonZero, time::Duration};

use audio_playback::media_probe::audio_metadata::extract_audio_metadata;
use symphonia::core::{
    audio::Channels,
    codecs::{
        CodecParameters,
        audio::{AudioCodecParameters, well_known::CODEC_ID_FLAC},
    },
    formats::{Track, TrackFlags},
    units::{Duration as SymphoniaDuration, TimeBase},
};

fn stub_track(
    codec_params: Option<CodecParameters>,
    num_frames: Option<u64>,
    time_base: Option<TimeBase>,
    duration: Option<u64>,
) -> Track {
    let duration = duration.map(|duration| SymphoniaDuration::new(duration));
    Track {
        id: 1,
        codec_params,
        language: None,
        time_base,
        num_frames,
        duration,
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

/// @relation(APLLSR-012-T01, scope=function)
#[test]
fn extracts_flac_channel_count() {
    let codec_params =
        flac_codec_params(Some(Channels::Discrete(2)), Some(44_100), Some(16));
    let track = stub_track(Some(codec_params), None, None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.channels, Some(2));
}

/// @relation(APLLSR-014-T01, scope=function)
#[test]
fn extracts_flac_sample_rate() {
    let codec_params = flac_codec_params(None, Some(44_100), Some(16));
    let track = stub_track(Some(codec_params), None, None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.sample_rate, Some(44_100));
}

/// @relation(APLLSR-016-T01, scope=function)
#[test]
fn extracts_flac_bits_per_sample() {
    let codec_params = flac_codec_params(None, Some(44_100), Some(24));
    let track = stub_track(Some(codec_params), None, None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.bits_per_sample, Some(24));
}

/// @relation(APLLSR-018-T01, scope=function)
#[test]
fn calculates_flac_duration_using_sample_frames_and_sample_rate() {
    let codec_params = flac_codec_params(None, Some(44_100), Some(16));
    let track = stub_track(Some(codec_params), Some(44_100), None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.duration, Some(Duration::from_secs(1)));
}

/// @relation(APLLSR-019-T01, scope=function)
#[test]
fn calculates_flac_duration_using_time_base_when_sample_duration_is_unavailable()
 {
    let codec_params = flac_codec_params(None, Some(44_100), Some(16));
    let track = stub_track(
        Some(codec_params),
        None,
        Some(TimeBase::new(
            NonZero::try_from(1).unwrap(),
            NonZero::try_from(1_000).unwrap(),
        )),
        Some(5_000),
    );

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.duration, Some(Duration::from_secs(5)));
}

/// @relation(APLLSR-018-T02, scope=function)
#[test]
fn prefers_sample_based_duration_when_both_duration_sources_are_available() {
    let codec_params = flac_codec_params(None, Some(44_100), Some(16));
    let track = stub_track(
        Some(codec_params),
        Some(44_100),
        Some(TimeBase::new(
            NonZero::try_from(1).unwrap(),
            NonZero::try_from(1_000).unwrap(),
        )),
        Some(5_000),
    );

    let metadata = extract_audio_metadata(&track).unwrap();

    // Sample-based duration is 1 second; time-base duration is 5 seconds.
    assert_eq!(metadata.duration, Some(Duration::from_secs(1)));
}

/// @relation(APLLSR-020-T01, scope=function)
#[test]
fn returns_unknown_duration_when_neither_duration_source_is_available() {
    let codec_params = flac_codec_params(None, None, Some(16));
    let track = stub_track(Some(codec_params), None, None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert_eq!(metadata.duration, None);
}

#[test]
fn returns_default_metadata_when_codec_parameters_are_missing() {
    let track = stub_track(None, None, None, None);

    let metadata = extract_audio_metadata(&track).unwrap();

    assert!(metadata.channels.is_none());
    assert!(metadata.sample_rate.is_none());
    assert!(metadata.bits_per_sample.is_none());
    assert!(metadata.duration.is_none());
}
