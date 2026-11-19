use audio_playback::media_probe::{
    AudioStreamError, AudioTrackInfo, ProbedAudioReader,
    identify_classification,
};
use symphonia::core::codecs::{
    CodecParameters,
    audio::{AudioCodecParameters, well_known::CODEC_ID_FLAC},
};

struct StubProbedAudioReader {
    format: String,
    track_id: Option<u32>,
    codec_params: Option<CodecParameters>,
}

impl ProbedAudioReader for StubProbedAudioReader {
    fn format_short_name(&self) -> &str {
        &self.format
    }

    fn first_audio_track(
        &self,
    ) -> Option<audio_playback::media_probe::AudioTrackInfo<'_>> {
        self.track_id.map(|id| AudioTrackInfo {
            id,
            codec_params: self.codec_params.as_ref(),
        })
    }
}

fn stub_reader(
    format: &str,
    track_id: Option<u32>,
    codec_params: Option<CodecParameters>,
) -> StubProbedAudioReader {
    StubProbedAudioReader {
        format: format.to_owned(),
        track_id,
        codec_params,
    }
}

fn flac_codec_params() -> CodecParameters {
    let mut params = AudioCodecParameters::new();
    params.for_codec(CODEC_ID_FLAC);

    CodecParameters::Audio(params)
}

/// @relation(APLLSR-001-T01, scope=function)
#[test]
fn identifies_flac_format_and_encoding() {
    let codec_params = flac_codec_params();
    let reader = stub_reader("flac", Some(1), Some(codec_params));

    let (classification, track) = identify_classification(&reader).unwrap();

    assert_eq!(classification.format, "flac");
    assert_eq!(classification.encoding, CODEC_ID_FLAC);
    assert_eq!(track.id, 1);
}

/// @relation(APLLSR-001-T02, scope=function)
#[test]
fn preserves_audio_track_id() {
    let codec_params = flac_codec_params();
    let reader = stub_reader("flac", Some(42), Some(codec_params));

    let (_, track) = identify_classification(&reader).unwrap();

    assert_eq!(track.id, 42);
}

/// @relation(APLLSR-009-T01, scope=function)
#[test]
fn returns_no_audio_track_when_no_track_exists() {
    let reader = stub_reader("flac", None, None);

    let result = identify_classification(&reader);

    assert!(matches!(result, Err(AudioStreamError::NoAudioTrack)));
}
