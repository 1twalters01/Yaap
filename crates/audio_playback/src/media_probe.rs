use std::fmt;

use symphonia::core::{
    codecs::{
        CodecParameters::{self, Audio},
        audio::{
            AudioCodecId, CODEC_ID_NULL_AUDIO, well_known::CODEC_ID_FLAC,
        },
    },
    errors::Error as SymphoniaError,
    formats::{FormatOptions, FormatReader, TrackType, probe::Hint},
    io::MediaSourceStream,
    meta::MetadataOptions,
};

pub struct IdentifiedAudioStream<'a> {
    pub classification: AudioStreamClassification,
    pub track_id: u32,
    pub reader: Box<dyn FormatReader + 'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct AudioStreamClassification {
    pub format: String,
    pub encoding: AudioCodecId,
}

#[derive(Debug, thiserror::Error)]
pub enum AudioStreamError {
    #[error("Audio format error: {0}")]
    Format(AudioFormatError),

    #[error("Audio encoding error: {0}")]
    Encoding(AudioEncodingError),

    #[error("Audio format and encoding error: {0}")]
    FormatAndEncoding(AudioFormatAndEncodingError),

    // @relation(APLLSR-009, scope=line)
    #[error("No audio track was found")]
    NoAudioTrack,

    #[error("Audio track not found: {0:?}")]
    AudioTrackNotFound(AudioTrackSelection),

    #[error("Audio stream I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symphonia error: {0}")]
    Symphonia(#[from] SymphoniaError),
}

#[derive(Debug)]
pub struct AudioFormatAndEncodingError {
    pub format_error: AudioFormatError,
    pub encoding_error: AudioEncodingError,
}

impl fmt::Display for AudioFormatAndEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.format_error, self.encoding_error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AudioFormatError {
    #[error("Unknown audio format: {0}")]
    Unknown(String),

    #[error("Unsupported audio format: {0}")]
    Unsupported(String),

    #[error("Invalid audio format: {0}")]
    Invalid(String),
}

#[derive(Debug, thiserror::Error)]
pub enum AudioEncodingError {
    #[error("Unknown audio encoding: {0}")]
    Unknown(String),

    #[error("Unsupported audio encoding: {0}")]
    Unsupported(String),

    #[error("Invalid audio encoding: {0}")]
    Invalid(String),
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

struct SymphoniaProbedReader {
    format_short_name: String,
    audio_tracks: Vec<(u32, Option<CodecParameters>)>,
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

pub struct ProbedAudioStream<'a> {
    pub probed_reader: Box<dyn ProbedAudioReader + 'a>,
    pub reader: Box<dyn FormatReader + 'a>,
}

pub trait AudioProber {
    fn probe_stream<'a>(
        &self,
        media_source_stream: MediaSourceStream<'a>,
    ) -> Result<ProbedAudioStream<'a>, SymphoniaError>;
}

impl Default for SymphoniaProber {
    fn default() -> Self {
        Self {
            hint: Hint::new(),
            format_options: FormatOptions::default(),
            metadata_options: MetadataOptions::default(),
        }
    }
}

pub struct SymphoniaProber {
    pub hint: Hint,
    pub format_options: FormatOptions,
    pub metadata_options: MetadataOptions,
}

impl AudioProber for SymphoniaProber {
    fn probe_stream<'a>(
        &self,
        media_source_stream: MediaSourceStream<'a>,
    ) -> Result<ProbedAudioStream<'a>, SymphoniaError> {
        let reader = symphonia::default::get_probe().probe(
            &self.hint,
            media_source_stream,
            self.format_options.clone(),
            self.metadata_options,
        )?;

        let format_short_name = reader.format_info().short_name.to_owned();
        let audio_tracks = reader
            .tracks()
            .iter()
            .filter(|track| {
                matches!(track.codec_params, Some(CodecParameters::Audio(_)))
            })
            .map(|track| (track.id, track.codec_params.clone()))
            .collect();

        let probed_reader = Box::new(SymphoniaProbedReader {
            format_short_name,
            audio_tracks,
        });

        Ok(ProbedAudioStream {
            probed_reader,
            reader,
        })
    }
}

#[derive(Debug)]
pub enum AudioTrackSelection {
    First,
    Index(usize),
    Id(u32),
}

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

/// @relation(APLLSR-001, scope=function)
fn identify_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
    selection: AudioTrackSelection,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
    let probed = prober
        .probe_stream(media_source_stream)
        .map_err(|error| map_probe_error(error))?;

    let (classification, track) =
        identify_classification(probed.probed_reader.as_ref(), selection)?;

    Ok(IdentifiedAudioStream {
        classification,
        track_id: track.id,
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

pub fn identify_classification(
    probed_reader: &dyn ProbedAudioReader,
    selection: AudioTrackSelection,
) -> Result<(AudioStreamClassification, AudioTrackInfo<'_>), AudioStreamError>
{
    let format = probed_reader.format_short_name().to_owned();
    let track = probed_reader.select_audio_track(selection)?;

    let classification = extract_classification(format, track.codec_params)?;

    Ok((classification, track))
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
        Audio(audio) => audio,
        // @relation(APLLSR-006, scope=line)
        _ => {
            return Err(AudioStreamError::Encoding(
                AudioEncodingError::Unknown(
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

pub fn verify_format_supported(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    let format_result = validate_audio_format(&classification.format);
    let encoding_result = validate_audio_encoding(classification.encoding);

    match (format_result, encoding_result) {
        (Ok(()), Ok(())) => {
            validate_format_encoding_combination(classification)
        }
        (
            Err(AudioStreamError::Format(format)),
            Err(AudioStreamError::Encoding(encoding)),
        ) => Err(AudioStreamError::FormatAndEncoding(
            AudioFormatAndEncodingError {
                format_error: format,
                encoding_error: encoding,
            },
        )),
        (Err(error), _) | (_, Err(error)) => Err(error),
    }
}

pub fn validate_audio_format(format: &str) -> Result<(), AudioStreamError> {
    match format {
        "flac" => Ok(()),
        // @relation(APLLSR-003, scope=line)
        other => Err(AudioStreamError::Format(AudioFormatError::Unsupported(
            format!("Audio format '{other}' is not supported"),
        ))),
    }
}

pub fn validate_audio_encoding(
    encoding: AudioCodecId,
) -> Result<(), AudioStreamError> {
    match encoding {
        CODEC_ID_NULL_AUDIO => {
            // @relation(APLLSR-006, scope=line)
            Err(AudioStreamError::Encoding(AudioEncodingError::Unknown(
                "Audio codec identifier is null".to_owned(),
            )))
        }

        // @relation(APLLSR-010, scope=line)
        CODEC_ID_FLAC => Ok(()),

        // @relation(APLLSR-004, scope=line)
        other => {
            Err(AudioStreamError::Encoding(AudioEncodingError::Unsupported(
                format!("Audio encoding '{other}' is not supported"),
            )))
        }
    }
}

pub fn validate_format_encoding_combination(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    match (classification.format.as_str(), classification.encoding) {
        // @relation(APLLSR-011, scope=line)
        ("flac", CODEC_ID_FLAC) => Ok(()),

        // @relation(APLLSR-004, scope=line)
        (format, encoding) => Err(AudioStreamError::Encoding(
            AudioEncodingError::Unsupported(format!(
                "Audio encoding '{encoding}' is not supported in audio format '{format}'"
            )),
        )),
    }
}
