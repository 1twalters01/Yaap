use symphonia::core::{codecs::CodecParameters, errors::Error as SymphoniaError, formats::{FormatOptions, FormatReader, probe::Hint}, io::MediaSourceStream, meta::MetadataOptions};

use crate::media_probe::reader::{ProbedAudioReader, SymphoniaProbedReader};

pub trait AudioProber {
    fn probe_stream<'a>(
        &self,
        media_source_stream: MediaSourceStream<'a>,
    ) -> Result<ProbedAudioStream<'a>, SymphoniaError>;
}

pub struct ProbedAudioStream<'a> {
    pub probed_reader: Box<dyn ProbedAudioReader + 'a>,
    pub reader: Box<dyn FormatReader + 'a>,
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
            .cloned()
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

impl Default for SymphoniaProber {
    fn default() -> Self {
        Self {
            hint: Hint::new(),
            format_options: FormatOptions::default(),
            metadata_options: MetadataOptions::default(),
        }
    }
}
