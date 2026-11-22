use std::time::Duration;

use symphonia::core::codecs::CodecParameters;

#[derive(Debug, Default)]
pub struct AudioStreamMetadata {
    pub channels: Option<u16>,
    pub sample_rate: Option<u32>,
    pub bits_per_sample: Option<u32>,
    pub duration: Option<Duration>,
}

pub fn extract_audio_metadata(codec_params: &CodecParameters) -> AudioStreamMetadata {
    AudioStreamMetadata::default()
}
