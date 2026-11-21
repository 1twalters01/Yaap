use symphonia::core::{codecs::CodecParameters, formats::{FormatReader, TrackType}};

use crate::media_probe::errors::AudioStreamError;

#[derive(Debug)]
pub enum AudioTrackSelection {
    First,
    Index(usize),
    Id(u32),
}

pub struct AudioTrackInfo<'a> {
    pub index: usize,
    pub id: u32,
    pub codec_params: Option<&'a CodecParameters>,
}

pub trait ProbedAudioReader {
    fn format_short_name(&self) -> &str;
    fn audio_tracks(&self) -> Vec<AudioTrackInfo<'_>>;

    fn select_audio_track(
        &self,
        selection: AudioTrackSelection,
    ) -> Result<AudioTrackInfo<'_>, AudioStreamError> {
        let tracks = self.audio_tracks();

        if tracks.is_empty() {
            return Err(AudioStreamError::NoAudioTrack);
        }

        let track = match selection {
            AudioTrackSelection::First => tracks.into_iter().next(),

            AudioTrackSelection::Index(index) => {
                tracks.into_iter().find(|track| track.index == index)
            }

            AudioTrackSelection::Id(id) => {
                tracks.into_iter().find(|track| track.id == id)
            }
        };

        track.ok_or(AudioStreamError::AudioTrackNotFound(selection))
    }
}

impl<T: FormatReader + ?Sized> ProbedAudioReader for T {
    fn format_short_name(&self) -> &str {
        self.format_info().short_name
    }

    fn audio_tracks(&self) -> Vec<AudioTrackInfo<'_>> {
        self.first_track(TrackType::Audio)
            .iter()
            .filter(|track| {
                track.codec_params.is_some()
                    && matches!(
                        track.codec_params.as_ref(),
                        Some(CodecParameters::Audio(_))
                    )
            })
            .enumerate()
            .map(|(index, track)| AudioTrackInfo {
                index,
                id: track.id,
                codec_params: track.codec_params.as_ref(),
            })
            .collect()
    }
}

pub(crate) struct SymphoniaProbedReader {
    pub(crate) format_short_name: String,
    pub(crate) audio_tracks: Vec<(u32, Option<CodecParameters>)>,
}

impl ProbedAudioReader for SymphoniaProbedReader {
    fn format_short_name(&self) -> &str {
        &self.format_short_name
    }

    fn audio_tracks(&self) -> Vec<AudioTrackInfo<'_>> {
        self.audio_tracks
            .iter()
            .enumerate()
            .map(|(index, (id, codec_params))| AudioTrackInfo {
                index,
                id: *id,
                codec_params: codec_params.as_ref(),
            })
            .collect()
    }
}

