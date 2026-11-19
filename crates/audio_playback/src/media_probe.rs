use symphonia::core::{
    codecs::{
        CodecParameters::{self, Audio},
        audio::{AudioCodecId, CODEC_ID_NULL_AUDIO, well_known::CODEC_ID_FLAC},
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
    // @relation(APLLSR-005, scope=line)
    #[error("Unknown audio format")]
    UnknownFormat,

    // @relation(APLLSR-003, scope=line)
    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),

    // @relation(APLLSR-007, scope=line)
    #[error("Invalid audio format: {0}")]
    InvalidFormat(String),

    // @relation(APLLSR-006, scope=line)
    #[error("Unknown audio encoding")]
    UnknownEncoding,

    // @relation(APLLSR-004, scope=line)
    #[error("Unsupported audio encoding: {0}")]
    UnsupportedEncoding(String),

    // @relation(APLLSR-008, scope=line)
    #[error("Invalid audio encoding: {0}")]
    InvalidEncoding(String),

    // @relation(APLLSR-009, scope=line)
    #[error("No audio track was found")]
    NoAudioTrack,

    #[error("Audio stream I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Symphonia error: {0}")]
    Symphonia(#[from] SymphoniaError),
}

pub struct AudioTrackInfo<'a> {
    pub id: u32,
    pub codec_params: Option<&'a CodecParameters>,
}

pub trait ProbedAudioReader {
    fn format_short_name(&self) -> &str;
    fn first_audio_track(&self) -> Option<AudioTrackInfo<'_>>;
}

impl<T: FormatReader + ?Sized> ProbedAudioReader for T {
    fn format_short_name(&self) -> &str {
        self.format_info().short_name
    }

    fn first_audio_track(&self) -> Option<AudioTrackInfo<'_>> {
        self.first_track(TrackType::Audio)
            .map(|track| AudioTrackInfo {
                id: track.id,
                codec_params: track.codec_params.as_ref(),
            })
    }
}

struct SymphoniaProbedReader {
    format_short_name: String,
    audio_track: Option<(u32, Option<CodecParameters>)>,
}

impl ProbedAudioReader for SymphoniaProbedReader {
    fn format_short_name(&self) -> &str {
        &self.format_short_name
    }

    fn first_audio_track(&self) -> Option<AudioTrackInfo<'_>> {
        self.audio_track
            .as_ref()
            .map(|(id, codec_params)| AudioTrackInfo {
                id: *id,
                codec_params: codec_params.as_ref(),
            })
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
        let audio_track = reader
            .first_track(TrackType::Audio)
            .map(|track| (track.id, track.codec_params.clone()));

        let probed_reader = Box::new(SymphoniaProbedReader {
            format_short_name,
            audio_track,
        });

        Ok(ProbedAudioStream {
            probed_reader,
            reader,
        })
    }
}

// @relation(APLLSR-002, scope=function)
pub fn identify_supported_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
    let identified_audio_stream = identify_audio_stream(media_source_stream, prober)?;
    verify_format_supported(&identified_audio_stream.classification)?;

    Ok(identified_audio_stream)
}

// @relation(APLLSR-001, scope=function)
fn identify_audio_stream<'a, P: AudioProber>(
    media_source_stream: MediaSourceStream<'a>,
    prober: &P,
) -> Result<IdentifiedAudioStream<'a>, AudioStreamError> {
    let probed = prober
        .probe_stream(media_source_stream)
        .map_err(|error| map_probe_error(error))?;

    let (classification, track) = identify_classification(probed.probed_reader.as_ref())?;

    Ok(IdentifiedAudioStream {
        classification,
        track_id: track.id,
        reader: probed.reader,
    })
}

fn map_probe_error(error: SymphoniaError) -> AudioStreamError {
    match error {
        // @relation(APLLSR-005, scope=line)
        SymphoniaError::Unsupported(_) => AudioStreamError::UnknownFormat,
        SymphoniaError::DecodeError(message) => {
            // @relation(APLLSR-007, scope=line)
            AudioStreamError::InvalidFormat(message.to_string())
        }

        SymphoniaError::IoError(error) => AudioStreamError::Io(error),
        other => AudioStreamError::Symphonia(other),
    }
}

fn identify_classification(
    probed_reader: &dyn ProbedAudioReader,
) -> Result<(AudioStreamClassification, AudioTrackInfo<'_>), AudioStreamError> {
    let format = probed_reader.format_short_name().to_owned();
    let track = probed_reader
        .first_audio_track()
        .ok_or(AudioStreamError::NoAudioTrack)?;

    let classification = extract_classification(format, track.codec_params)?;

    Ok((classification, track))
}

fn extract_classification(
    format: String,
    codec_params: Option<&CodecParameters>,
) -> Result<AudioStreamClassification, AudioStreamError> {
    // @relation(APLLSR-006, scope=line)
    let params = codec_params.ok_or(AudioStreamError::UnknownEncoding)?;

    let audio_params = match params {
        Audio(audio) => audio,
        // @relation(APLLSR-006, scope=line)
        _ => return Err(AudioStreamError::UnknownEncoding),
    };

    if audio_params.codec == CODEC_ID_NULL_AUDIO {
        // @relation(APLLSR-006, scope=line)
        return Err(AudioStreamError::UnknownEncoding);
    }

    Ok(AudioStreamClassification {
        format,
        encoding: audio_params.codec,
    })
}

fn verify_format_supported(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    validate_audio_format(&classification.format)?;
    validate_audio_encoding(classification.encoding)?;
    validate_format_encoding_combination(classification)?;

    Ok(())
}

fn validate_audio_format(format: &str) -> Result<(), AudioStreamError> {
    match format {
        "flac" => Ok(()),
        // @relation(APLLSR-003, scope=line)
        other => Err(AudioStreamError::UnsupportedFormat(other.to_owned())),
    }
}

fn validate_audio_encoding(encoding: AudioCodecId) -> Result<(), AudioStreamError> {
    match encoding {
        // @relation(APLLSR-010, scope=line)
        CODEC_ID_FLAC => Ok(()),

        // @relation(APLLSR-004, scope=line)
        other => Err(AudioStreamError::UnsupportedEncoding(format!("{other:?}"))),
    }
}

fn validate_format_encoding_combination(
    classification: &AudioStreamClassification,
) -> Result<(), AudioStreamError> {
    match (classification.format.as_str(), classification.encoding) {
        // @relation(APLLSR-011, scope=line)
        ("flac", CODEC_ID_FLAC) => Ok(()),

        // @relation(APLLSR-004, scope=line)
        _ => Err(AudioStreamError::UnsupportedEncoding(format!(
            "Encoding {:?} is not supported in format {}",
            classification.encoding, classification.format,
        ))),
    }
}
